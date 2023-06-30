# File Previews

`joshuto` uses preview similar files compared to `ranger`.

## `joshuto.toml`

First, within `joshuto.toml`, there is a `preview` section.
```toml
[preview]

# This is the maximum size (in bytes) of a file to generate a preview for.
# Any file greater than this amount will not have a preview generated.
max_preview_size = 2097152 # 2MB

# This is the script that will be ran whenever a preview needs to be generated
preview_script = "~/.config/joshuto/preview_file.sh"

preview_shown_hook_script = "~/.config/joshuto/on_preview_shown.sh"

# This script is ran whenever the preview selection changes.
# Usually used to cleanup the old preview.
preview_removed_hook_script = "~/.config/joshuto/on_preview_removed.sh"
```

## `preview_script`
**This file MUST be executable to work**

This script is designed very similarly to `ranger`'s preview scripts with
a few changes to make it easier to use.

`preview_script` take the following command line arguments
```shell
--path <path_to_file>   # this is the oath of the file
                        # we want to generate a preview for

--preview-width <width>     # the width of the preview
                            # we want to generate

--preview-height <height>   # the height of the preview
                            # we want to generate

--x-coord x     # the x coordinate of the terminal
                # to preview will be shown

--y-coord y     # the y coordinate of the terminal
                # to preview will be shown

--preview-images <1|0>  # to enable image previews (1) or not (0)

--image-cache <path>    # setup a image cache directory to be used for
                        # storing previews to reduce computation
```

To learn more about the implementation, see example [here](/config/preview_file.sh)

### Exit codes

`preview_script` will exit with a specific exit code to indicate to
`joshuto` the result of running the preview script.

It follows `ranger`'s exit codes for the most part.

**NOTE: Not all exit codes have been extensively implemented and tested, so there might be some feature gaps from ranger**

```python
## Meanings of exit codes:
## code | meaning	    | action of joshuto
## -----+------------+-------------------------------------------
## 0	| success	    | Display stdout as preview
## 1	| no preview	| Display no preview at all
## 2	| plain text	| Display the plain content of the file
## 3	| fix width 	| Don't reload when width changes
## 4	| fix height	| Don't reload when height changes
## 5	| fix both	    | Don't ever reload
## 6	| images	    | Display the image `$IMAGE_CACHE_PATH` points to as an image preview
## 7	| images	    | Display the file directly as an image
```

## `preview_shown_hook_script`

## `preview_removed_hook_script`

## Image previews

See [image_previews.md](/docs/image_previews.md)