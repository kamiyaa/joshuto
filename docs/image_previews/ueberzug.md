# Image Previews with Überzug
This recipe shows how to use [Überzug](https://github.com/seebye/ueberzug) to display image previews.
Be aware that the Überzug project is archived and not maintained anymore.

Überzug must be installed for the solution explained here.

## Joshuto Wrapper

First, we need a wrapper script for Joshuto.
Joshuto is not started directly anymore but by that wrapper script
after Überzug has been started.
Place this script in a directory which is in your `$PATH`.
You may name the script `j` or `jo` or whatever you like to type to start Joshuto.
You may also name it `joshuto`, just take care that it
must come before the actual `joshuto` binary in you `$PATH` in that case and
that the `joshuto` call down in the script calls the actual `joshuto` binary.

```bash
#!/usr/bin/env bash

if [ -n "$DISPLAY" ] && command -v ueberzug > /dev/null; then
    export joshuto_wrap_id="$$"
    export joshuto_wrap_tmp="$(mktemp -d -t joshuto-wrap-$joshuto_wrap_id-XXXXXX)"
    export joshuto_wrap_ueber_fifo="$joshuto_wrap_tmp/fifo"
    export joshuto_wrap_pid_file="$joshuto_wrap_tmp/pid"
    export joshuto_wrap_preview_meta="$joshuto_wrap_tmp/preview-meta"
    export joshuto_wrap_ueber_identifier="preview"

    function start_ueberzug {
        mkfifo "${joshuto_wrap_ueber_fifo}"
        tail --follow "$joshuto_wrap_ueber_fifo" | ueberzug layer  --parser bash &
        echo "$!" > "$joshuto_wrap_pid_file"
        mkdir -p "$joshuto_wrap_preview_meta"
    }

    function stop_ueberzug {
        ueberzug_pid=`cat "$joshuto_wrap_pid_file"`
        kill "$ueberzug_pid"
        rm -rf "$joshuto_wrap_tmp"
    }

    function show_image {
        >"${joshuto_wrap_ueber_fifo}" declare -A -p cmd=( \
                [action]=add [identifier]="${joshuto_wrap_ueber_identifier}" \
                [x]="${2}" [y]="${3}" \
                [width]="${4}" [height]="${5}" \
                [path]="${1}")
    }

    function remove_image {
        >"${joshuto_wrap_ueber_fifo}" declare -A -p cmd=( \
            [action]=remove [identifier]="${joshuto_wrap_ueber_identifier}")
    }

    function get_preview_meta_file {
        echo "$joshuto_wrap_preview_meta/$(echo "$1" | md5sum | sed 's/ //g')"
    }

    export -f get_preview_meta_file
    export -f show_image
    export -f remove_image

    trap stop_ueberzug EXIT QUIT INT TERM
    start_ueberzug
    echo "ueberzug started"
fi

joshuto "$@"
exit $?
```

The script must be _executable_!

This script starts an “ueberzug server” and then Joshuto itself.
It takes care that ueberzug is stopped when
`joshuto` terminates.
Each Joshuto instance will have its own instance of an “ueberzug server”.
The script also provides some functions
and variables which can be used in sub-processes.

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

