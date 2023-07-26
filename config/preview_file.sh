#!/usr/bin/env bash

## This script is a template script for creating textual file previews in Joshuto.
##
## Copy this script to your Joshuto configuration directory and refer to this
## script in `joshuto.toml` in the `[preview]` section like
## ```
## preview_script = "~/.config/joshuto/preview_file.sh"
## ```
## Make sure the file is marked as executable:
## ```sh
## chmod +x ~/.config/joshuto/preview_file.sh
## ```
## Joshuto will call this script for each file when first hovered by the cursor.
## If this script returns with an exit code 0, the stdout of this script will be
## the file's preview text in Joshuto's right panel.
## The preview text will be cached by Joshuto and only renewed on reload.
## ANSI color codes are supported if Joshuto is build with the `syntax_highlight`
## feature.
##
## This script is considered a configuration file and must be updated manually.
## It will be left untouched if you upgrade Joshuto.
##
## Meanings of exit codes:
##
## code | meaning    | action of ranger
## -----+------------+-------------------------------------------
## 0    | success    | Display stdout as preview
## 1    | no preview | Display no preview at all
##
## This script is used only as a provider for textual previews.
## Image previews are independent from this script.
##

IFS=$'\n'

# Security measures:
# * noclobber prevents you from overwriting a file with `>`
# * noglob prevents expansion of wild cards
# * nounset causes bash to fail if an undeclared variable is used (e.g. typos)
# * pipefail causes a pipeline to fail also if a command other than the last one fails
set -o noclobber -o noglob -o nounset -o pipefail

FILE_PATH=""
PREVIEW_WIDTH=10
PREVIEW_HEIGHT=10

while [ "$#" -gt 0 ]; do
    case "$1" in
        "--path")
            shift
            FILE_PATH="$1"
            ;;
        "--preview-width")
            shift
            PREVIEW_WIDTH="$1"
            ;;
        "--preview-height")
            shift
            PREVIEW_HEIGHT="$1"
            ;;
    esac
    shift
done

handle_extension() {
    case "${FILE_EXTENSION_LOWER}" in
            ## Archive
            a|ace|alz|arc|arj|bz|bz2|cab|cpio|deb|gz|jar|lha|lz|lzh|lzma|lzo|\
            rpm|rz|t7z|tar|tbz|tbz2|tgz|tlz|txz|tZ|tzo|war|xpi|xz|Z|zip)
            atool --list -- "${FILE_PATH}" && exit 0
            bsdtar --list --file "${FILE_PATH}" && exit 0
            exit 1 ;;
        rar)
            ## Avoid password prompt by providing empty password
            unrar lt -p- -- "${FILE_PATH}" && exit 0
            exit 1 ;;
        7z)
            ## Avoid password prompt by providing empty password
            7z l -p -- "${FILE_PATH}" && exit 0
            exit 1 ;;

            ## PDF
        pdf)
            ## Preview as text conversion
            pdftotext -l 10 -nopgbrk -q -- "${FILE_PATH}" - | \
                fmt -w "${PREVIEW_WIDTH}" && exit 0
            mutool draw -F txt -i -- "${FILE_PATH}" 1-10 | \
                fmt -w "${PREVIEW_WIDTH}" && exit 0
            exiftool "${FILE_PATH}" && exit 0
            exit 1 ;;

            ## BitTorrent
        torrent)
            transmission-show -- "${FILE_PATH}" && exit 0
            exit 1 ;;

            ## OpenDocument
        odt|sxw)
            ## Preview as text conversion
            odt2txt "${FILE_PATH}" && exit 0
            ## Preview as markdown conversion
            pandoc -s -t markdown -- "${FILE_PATH}" && exit 0
            exit 1 ;;
        ods|odp)
            ## Preview as text conversion (unsupported by pandoc for markdown)
            odt2txt "${FILE_PATH}" && exit 0
            exit 1 ;;

            ## XLSX
        xlsx)
            ## Preview as csv conversion
            ## Uses: https://github.com/dilshod/xlsx2csv
            xlsx2csv -- "${FILE_PATH}" && exit 0
            exit 1 ;;

            ## HTML
        htm|html|xhtml)
            ## Preview as text conversion
            w3m -dump "${FILE_PATH}" && exit 0
            lynx -dump -- "${FILE_PATH}" && exit 0
            elinks -dump "${FILE_PATH}" && exit 0
            pandoc -s -t markdown -- "${FILE_PATH}" && exit 0
            ;;

            ## JSON
        json|ipynb)
            jq --color-output . "${FILE_PATH}" && exit 0
            python -m json.tool -- "${FILE_PATH}" && exit 0
            ;;

            ## Direct Stream Digital/Transfer (DSDIFF) and wavpack aren't detected
            ## by file(1).
        dff|dsf|wv|wvc)
            mediainfo "${FILE_PATH}" && exit 0
            exiftool "${FILE_PATH}" && exit 0
            ;; # Continue with next handler on failure
    esac
}

handle_mime() {
    local mimetype="${1}"

    case "${mimetype}" in
            ## RTF and DOC
        text/rtf|*msword)
            ## Preview as text conversion
            ## note: catdoc does not always work for .doc files
            ## catdoc: http://www.wagner.pp.ru/~vitus/software/catdoc/
            catdoc -- "${FILE_PATH}" && exit 0
            exit 1 ;;

            ## DOCX, ePub, FB2 (using markdown)
            ## You might want to remove "|epub" and/or "|fb2" below if you have
            ## uncommented other methods to preview those formats
        *wordprocessingml.document|*/epub+zip|*/x-fictionbook+xml)
            ## Preview as markdown conversion
            pandoc -s -t markdown -- "${FILE_PATH}" | bat -l markdown \
                --color=always --paging=never \
                --style=plain \
                --terminal-width="${PREVIEW_WIDTH}" && exit 0
            exit 1 ;;

            ## E-mails
        message/rfc822)
            ## Parsing performed by mu: https://github.com/djcb/mu
            mu view -- "${FILE_PATH}" && exit 0
            exit 1 ;;

            ## XLS
        *ms-excel)
            ## Preview as csv conversion
            ## xls2csv comes with catdoc:
            ##   http://www.wagner.pp.ru/~vitus/software/catdoc/
            xls2csv -- "${FILE_PATH}" && exit 0
            exit 1 ;;

            ## Text
        text/* | */xml)
            bat --color=always --paging=never \
                --style=plain \
                --terminal-width="${PREVIEW_WIDTH}" \
                "${FILE_PATH}" && exit 0
            cat "${FILE_PATH}" && exit 0
            exit 1 ;;

            ## DjVu
        image/vnd.djvu)
            ## Preview as text conversion (requires djvulibre)
            djvutxt "${FILE_PATH}" | fmt -w "${PREVIEW_WIDTH}" && exit 0
            exiftool "${FILE_PATH}" && exit 0
            exit 1 ;;

            ## Image
        image/*)
            ## Preview as text conversion
            exiftool "${FILE_PATH}" && exit 0
            exit 1 ;;

            ## Video and audio
        video/* | audio/*)
            mediainfo "${FILE_PATH}" && exit 0
            exiftool "${FILE_PATH}" && exit 0
            exit 1 ;;
    esac
}

FILE_EXTENSION="${FILE_PATH##*.}"
FILE_EXTENSION_LOWER="$(printf "%s" "${FILE_EXTENSION}" | tr '[:upper:]' '[:lower:]')"
handle_extension
MIMETYPE="$( file --dereference --brief --mime-type -- "${FILE_PATH}" )"
handle_mime "${MIMETYPE}"

exit 1
