# Image Previews with Kitty's `icat`

The [Kitty](https://sw.kovidgoyal.net/kitty/) terminal must be [installed](https://sw.kovidgoyal.net/kitty/binary/#) 
and used for the solution explained here.

To preview images in Kitty, you need to create these two scripts and make them executable.

`~/.config/joshuto/on_preview_shown`:

```shell
#!/usr/bin/env bash

FILE_PATH="$1"			# Full path of the previewed file
PREVIEW_X_COORD="$2"		# x coordinate of upper left cell of preview area
PREVIEW_Y_COORD="$3"		# y coordinate of upper left cell of preview area
PREVIEW_WIDTH="$4"		# Width of the preview pane (number of fitting characters)
PREVIEW_HEIGHT="$5"		# Height of the preview pane (number of fitting characters)

TMP_FILE="$HOME/.cache/joshuto/thumbcache.png"

mimetype=$(file --mime-type -Lb "$FILE_PATH")

function image {
	kitty +kitten icat \
		--transfer-mode=file \
		--clear 2>/dev/null
	kitty +kitten icat \
		--transfer-mode=file \
		--place "${PREVIEW_WIDTH}x${PREVIEW_HEIGHT}@${PREVIEW_X_COORD}x${PREVIEW_Y_COORD}" \
		"$1" 2>/dev/null
}

case "$mimetype" in
	image/*)
		image "${FILE_PATH}"
		;;
	*)
		kitty +kitten icat \
			--transfer-mode=file \
			--clear 2>/dev/null
		;;
esac
```

`~/.config/joshuto/on_preview_removed.sh`:

```shell
#!/usr/bin/env bash

kitty +kitten icat \
	--transfer-mode=file \
	--clear 2>/dev/null
```

The first script will use `icat` to place an image on top of joshuto's preview window.
If any images already exist, they will be cleared before showing the image.

The second script simply clears any existing images on the screen.

That's it. Previewing images should now work whenever you select a file.

![Demo](https://user-images.githubusercontent.com/57725322/150659504-203c7175-4bee-4e46-b5c5-16cc16a51a12.png)

