# Image Previews with Überzug++
This recipe shows how to use [Überzug++](https://github.com/jstkdng/ueberzugpp)to display image previews.
Überzug++ is _not_ the famous Überzug, but a completely new project that tries to be compatible to 
“the old”, now unmaintained Überzug.

Überzug++ must be [installed](https://github.com/jstkdng/ueberzugpp#install) for the solution explained here.
In case of trouble, try first to get Überzug++ running standalone and check for parameters that
work for you.

This recipe is mostly the same as the Überzug recipe.
Only the wrapper script is different.

## Joshuto Wrapper

First, we need a wrapper script for Joshuto.
Joshuto is not started directly anymore but by that wrapper script
after Überzug has been started.
Place this script in a directory which is in your `$PATH`.
You may name the script `j` or `jo` or whatever you like to type to start Joshuto.
You may also name it `joshuto`, just take care that it
must come before the actual `joshuto` binary in you `$PATH` in that case and
that the `joshuto` call down in the script calls the actual `joshuto` binary.

The first line in the `start_ueberzugpp` function may need to be adapted to your
needs. It starts the actual `ueberzugpp` process that will show the images.
You may want to remove the `--no-opencv` switch and maybe add an output specifier
(like `-o kitty` to use the Kitty terminal backend).
Consult the Überzug++ documentation.

```bash
#!/usr/bin/env bash
#
## Example wrapper for using Überzug++

export joshuto_wrap_id="$$"
export joshuto_wrap_tmp="$(mktemp -d -t joshuto-wrap-$joshuto_wrap_id-XXXXXX)"
export joshuto_wrap_preview_meta="$joshuto_wrap_tmp/preview-meta"
export ueberzug_pid_file="$joshuto_wrap_tmp/pid"
export ueberzug_img_identifier="preview"
export ueberzug_socket=""
export ueberzug_pid=""


function start_ueberzugpp {
    ## Adapt Überzug++ options here. For example, remove the '--no-opencv' or set another output method.
    ueberzugpp layer --no-stdin --pid-file "$ueberzug_pid_file" --no-opencv &>/dev/null
    export ueberzug_pid="$(cat "$ueberzug_pid_file")"
    export ueberzug_socket=/tmp/ueberzugpp-"$ueberzug_pid".socket
    mkdir -p "$joshuto_wrap_preview_meta"
}

function stop_ueberzugpp {
    remove_image
    ueberzugpp cmd -s "$ueberzug_socket" -a exit
    kill "$ueberzug_pid"
    rm -rf "$joshuto_wrap_tmp"
}

function show_image {
    ueberzugpp cmd -s "$ueberzug_socket" -a add -i "$ueberzug_img_identifier" -x "$2" -y "$3" --max-width "$4" --max-height "$5" -f "$1" &>/dev/null
}

function remove_image {
    ueberzugpp cmd -s "$ueberzug_socket" -a remove -i "$ueberzug_img_identifier" &>/dev/null
}

function get_preview_meta_file {
    echo "$joshuto_wrap_preview_meta/$(echo "$1" | md5sum | sed 's/ //g')"
}

export -f get_preview_meta_file
export -f show_image
export -f remove_image
 
if [ -n "$DISPLAY" ] && command -v ueberzugpp > /dev/null; then
    trap stop_ueberzugpp EXIT QUIT INT TERM
    start_ueberzugpp
fi

joshuto "$@"
exit $?
```

The script must be _executable_!

## Configuring Hook Scripts

When started with the wrapper script, Joshuto's sub-processes can show and remove a
preview image with Überzug now. Joshuto offers two hooks which will be used for that.

In your `~/.config/joshuto/joshuto.toml`, configure a script for each of these hooks:

```toml
[preview]
...
preview_shown_hook_script = "~/.config/joshuto/on_preview_shown"
preview_removed_hook_script = "~/.config/joshuto/on_preview_removed"
```

## The Hook Scripts

Now we need to create the two hook scripts which have been configured before.

Create these two scripts and make them _executable_!

`~/.config/joshuto/on_preview_shown`:

```bash
#!/usr/bin/env bash

test -z "$joshuto_wrap_id" && exit 1;

path="$1"       # Full path of the previewed file
x="$2"          # x coordinate of upper left cell of preview area
y="$3"          # y coordinate of upper left cell of preview area
width="$4"      # Width of the preview pane (number of fitting characters)
height="$5"     # Height of the preview pane (number of fitting characters)


# Find out mimetype and extension
mimetype=$(file --mime-type -Lb "$path")
extension=$(/bin/echo "${path##*.}" | awk '{print tolower($0)}')

case "$mimetype" in
    image/png | image/jpeg)
        show_image "$path" $x $y $width $height
        ;;
    *)
        remove_image

esac
```

`~/.config/joshuto/on_preview_removed`:

```bash
#!/usr/bin/env bash
test -z "$joshuto_wrap_id" && exit 1;
remove_image
```

The first script shows a preview in case we have a JPEG or PNG file.
If there is already a preview image shown, it will just be replaced.
If we have a file other than JPEG or PNG, any preview which might be
visible is removed.

The second script just removes a preview image in case one is currently shown.

The removal of a preview in the first script is important when the user changes
the selection from an image file to a non-image file with text preview.

The removal in the second script is important when the user changes the selection
from an image file to a file without even a text-preview or a directory.

That's it. Previewing JPEG and PNG files should work now when the wrapper
script is started.

The `on_preview_shown` script may be extended for further mime-types.

