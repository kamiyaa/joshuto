# Thumbnails in File Previews

Joshuto supports some terminal graphics protocols for showing thumbnails out of the box.
See [ratatui-image](https://github.com/benjajaja/ratatui-image?tab=readme-ov-file#compatibility-matrix)
for which terminals are supported.

This works without further configuration if the [preview script](../file_previews.md) returns with `0` for a file.
The textual preview returned by the preview script will be shown right beneath the thumbnail image.
This allows to use both, an image preview and a textual preview which is often used to show meta-data for non-text files.

## Disabling thumbnails or changing the image protocol

This thumbnail feature is enabled by default and uses an auto-detection for the image-protocol to use.
To disable the inbuilt thumbnail feature or to override the detected protocol, the `preview_protocol` option can
be used in the `joshuto.toml` file. Accepted values are `auto` (default), 
`disabled`, or any of the [implemented protocols](https://docs.rs/ratatui-image/latest/ratatui_image/picker/enum.ProtocolType.html) 
in lowercase, for example:
```toml
[preview]
preview_protocol = "halfblocks"
...
```

## XDG-Thumbnails and thumbnails for non-image files

By default, Joshuto uses thumbnails and the
[freedesktop.org specified](https://specifications.freedesktop.org/thumbnail-spec/thumbnail-spec-latest.html)
thumbnail cache for the image previews.

This increases performance, because images are scaled down only once and then kept in a cache, which
can also be used by different programs.
This feature also allows to provide thumbnails for file formats other than image files.
Internally, Joshuto uses [allmytoes](https://gitlab.com/allmytoes/allmytoes) to provide the thumbnails.
Check the [documentation for “providers” there](https://gitlab.com/allmytoes/allmytoes#provider) to see which other
file types are supported and which local dependencies must be available to have them created when needed.

### Disabling XDG-Thumbnails

To disable the XDG-thumbnails and the provision of thumbs for non-image file types,
set the respective property in `joshuto.toml` to `false`:
```toml
[preview]
use_xdg_thumbs = false
```
The image-preview feature will still work, but will only show previews for image-files and it will
have to load the full-size images and then scale them down every time they are shown.

### Set the maximum size for thumbnails
The freedesktop.org thumbnail specification defines four different thumbnail sizes: normal, large, x-large, and xx-large
with maximum edge lengths of 128 px, 256 px, 512 px, and 1024 px respectively.

By default, Joshuto uses “x-large” thumbnails. This can be changed by the respective property in `joshuto.toml`, e.g.:
```toml
[preview]
# Allowed values are 'normal', 'large', 'xlarge', and 'xxlarge'
xdg_thumb_size = "xxlarge"
```

# Thumbnails via external scripts
The thumbnails provided directly by Joshuto might not be satisfying for all users.
Some terminals might not be supported at all, or only support a less satisfying image representation, like “halfblocks”.

As a solution for these cases,
Joshuto offers two text-preview-related hooks which allow to implement
an image preview with external scripts and some other “terminal image” solutions.
Most probably, you will use [Überzug++](ueberzugpp.md) for this job.
This allows, for example, to have proper preview-images in Alacritty, which otherwise only 
support “halfblocks” with Joshuto's inbuilt image-preview.

Essentially, you have to create two scripts and “hook” them into Joshuto.
Joshuto will call them every time a new image-preview shall be shown or when any preview-image shall
be cleared.

The hooks can be configured in the `joshuto.toml` file.
```toml
[preview]
...
preview_shown_hook_script = "~/.config/joshuto/preview_shown_hook"
preview_removed_hook_script = "~/.config/joshuto/preview_removed_hook"
```

Be aware that as soon as these scripts are configured, the inbuilt image-preview feature
is disabled automatically.

The `preview_shown_hook_script` is called each time a file-preview (in the 3rd 
pane) is focused.

The `preview_removed_hook_script` will be called each time the file preview 
panel completely disappears in Joshuto.
That is the case if the user selects a file for which no file preview is shown
(either due to missing output of the preview script or due to file size),
if the preview is not cached already and the preview pane is temporarily removed,
or if the selection jumps from a file to a directory.

The “shown” script gets the path of the file being previewed
as first argument and then the x and y coordinate and the width a
height of the preview area as second to fifth parameters.

The “removed” script does not get any arguments.

> ❗ **Important: *Text* preview area is what triggers the hook scripts** ❗
>
> The “preview” hooks are triggered by the [*text* preview](../file_previews.md).
> The “shown hook” will be invoked when the *text* preview area is shown.
> Ergo, a text preview must be activated for files for which one wants to have an
> image preview.
> If you don't want to do fancy stuff like combining a textual preview with image previews,
> just return with exit code `0` from the preview-script without any output for those files
> for which an image preview shall be shown.
> That will trigger Joshuto to show an empty text-preview area which can then be used
> as a canvas for the image preview.

## Wrapper Script
For some of the 3rd party tools, it's necessary 
to run them as a separate process, in parallel to Joshuto.

The famous example is “Überzug” and its successor “Überzug++”.
To be able to use such a solution,
one needs to have a “wrapper-script” which must be started instead of Joshuto.
The wrapper-script will then start both, first the program for showing images
in the terminal and then Joshuto.


## Recipes for external previews with Überzug(++)

See

* [Überzug](ueberzug.md) (only for X11; unmaintained but may live on in [some fork](https://github.com/ueber-devel/ueberzug))
* [Überzug++](ueberzugpp.md) (for X11, some Wayland compositors, and some specific terminal emulators)

## Tips and Tricks
* [Combining text preview and image preview](combined_with_text.md)
* To use XDG-thumbnails or thumbnails for non-image file types, you 
  can use the program [`allmytoes`](https://gitlab.com/allmytoes/allmytoes) inside
  your shown-hook script to get the thumb-image for a file.
  This is basically the “shell-program” version of what Joshuto also uses internally for the inbuilt XDG-thumbnail feature.

