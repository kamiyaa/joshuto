# Image Thumbnails in File Previews

Joshuto does not support image previews directly.
One reason is that Joshuto wants to stay independent of specific display protocols and terminal emulators.

However, Joshuto offers two text-preview-related hooks which allow to easily implement an
image preview with some simple scripts.
The hooks can be configured in the `joshuto.toml` file.
```toml
[preview]
...
preview_shown_hook_script = "~/path/to/some/executable_script_1"
preview_removed_hook_script = "~/path/to/some/executable_script_2"
```
The shown-hook is called whenever a new file-preview (in the 3rd pane) is activated.

The removed-hook will be called each time the file preview panel
completely disappears in Joshuto.
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

Using these hooks, one can trigger various actions when moving the cursor along files in Joshuto,
and they can also be used to show image previews by the help of other 3rd party tools.

Keep in mind that the result of the “normal” `preview` script you use for textual previews
is cached by Joshuto and is not called every time a file is focused, but the “shown” hook is.

# Wrapper Script
For some of the 3rd party tools, it's necessary 
to run them as a separate process, in parallel to Joshuto.

One famous example is “Überzug”. To be able to use such a solution,
one needs to have a “wrapper-script” which must be started instead of Joshuto.
The wrapper-script will then start both, first the program for showing images
in the terminal and then Joshuto.


# Recipes
## Image Thumbnail Solution Recipes
We have recipes for a few famous solutions.
* [Überzug](ueberzug.md) (only for X11)
* [Überzug++](ueberzugpp.md) (for X11, some Wayland compositors, and some specific terminal emulators)
* [Kitty](kitty.md) (for the Kitty terminal)

## Other Recipes and Tricks
* [Combining text preview and image preview](combined_with_text.md)

# Recipe Contributions welcome 🤗

Feel free to provide recipes to include in this documentation.
