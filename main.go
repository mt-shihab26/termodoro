package main

import (
	"github.com/rivo/tview"
)

func main() {
	app := tview.NewApplication()

	text := tview.NewTextView().
		SetText("Centered text").
		SetTextAlign(tview.AlignCenter)

	root := tview.NewFlex().
		SetDirection(tview.FlexRow).
		AddItem(nil, 0, 1, false).
		AddItem(text, 1, 0, false).
		AddItem(nil, 0, 1, false)

	app.SetRoot(root, true).Run()
}
