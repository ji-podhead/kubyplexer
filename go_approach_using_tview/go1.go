//item: Das tview.Primitive-Objekt, das Sie dem Grid hinzufügen möchten.
//row: Die Zeile, in der das Element platziert werden soll.
//column: Die Spalte, in der das Element platziert werden soll.
//rowSpan: Die Anzahl der Zeilen, die das Element überdecken soll.
//columnSpan: Die Anzahl der Spalten, die das Element überdecken soll.
//minWidth: Die minimale Breite des Elements in Zeichen.
//minHeight: Die minimale Höhe des Elements in Zeichen.
//expand: Ein boolescher Wert, der angibt, ob das Element vergrößert werden kann, wenn das Grid-Layout vergrößert wird.
//executeButton := tview.NewButton("Execute Command").
//SetSelectedFunc(func() {
//	// Führen Sie den Befehl aus und fangen Sie die Ausgabe ab
//	cmd := exec.Command("bash", "-c", "microk8s kubectl get pods -A")
//	var out bytes.Buffer
//	cmd.Stdout = &out
//	err := cmd.Run()
//	if err != nil {
//		outputView.SetText(fmt.Sprintf("Error: %v", err))
//	} else {
//		// Zeigen Sie die Ausgabe im TextView an
//		outputView.SetText(out.String())
//	}
//})                                  
//
package main

import (
	"bytes"
	"fmt"
	"os/exec"
	"strings"
	"github.com/rivo/tview"
	"github.com/gdamore/tcell/v2" // Import the v2 version of tcell
)


type CommandButton struct {
	Button *tview.Button
	Command string
	OutputView *tview.TextView
}

// NewCommandButton erstellt einen neuen CommandButton mit dem angegebenen Befehl und der Ausgabeansicht.
func NewCommandButton(command string, outputView *tview.TextView) *CommandButton {
	button := tview.NewButton("Execute Command").
		SetSelectedFunc(func() {
			// Führen Sie den Befehl aus und fangen Sie die Ausgabe ab
			cmd := exec.Command("bash", "-c", command)
			var out bytes.Buffer
			cmd.Stdout = &out
			err := cmd.Run()
			if err != nil {
				outputView.Write([]byte(fmt.Sprintf("Error: %v\n", err)))
			} else {
				// Fügen Sie die Ausgabe am Ende des bestehenden Inhalts an
				outputView.Write(out.Bytes())
			}
		})

	return &CommandButton{
		Button: button,
		Command: command,
		OutputView: outputView,
	}
}

// CustomScrollbar is a structure that represents a custom scrollbar.
type CustomScrollbar struct {
	TextView   *tview.TextView
	Scrollbar  *tview.TextView
	MaxLines   int
	CurrentLine int
}

func NewCustomScrollbar(textView *tview.TextView, app *tview.Application) *CustomScrollbar {
	scrollbar := tview.NewTextView().
		SetDynamicColors(true).
		SetRegions(true).
		SetChangedFunc(func() {
			app.Draw()
		})

	return &CustomScrollbar{
		TextView:  textView,
		Scrollbar: scrollbar,
		MaxLines:   0,
		CurrentLine:   0,
	}
}

func (cs *CustomScrollbar) Update() {
	// Calculate the number of lines in the TextView
	lines := strings.Count(cs.TextView.GetText(true), "\n") +   1

	// Update the MaxLines if necessary
	if lines > cs.MaxLines {
		cs.MaxLines = lines
	}

	// Calculate the height of the scrollbar
	scrollbarHeight := cs.TextView.Box.GetHeight() * cs.MaxLines / cs.TextView.Box.GetContentHeight()

	// Update the scrollbar display
	cs.Scrollbar.Clear()
	for i :=   0; i < scrollbarHeight; i++ {
		cs.Scrollbar.Write([]byte(" "))
	}
}


func (cs *CustomScrollbar) SetInputCapture(app *tview.Application) {
	cs.TextView.SetInputCapture(func(event *tcell.EventKey) *tcell.EventKey {
		switch event.Key() {
		case tcell.KeyUp: // Handle the scroll up event
			if cs.CurrentLine >   0 {
				cs.CurrentLine--
				cs.TextView.ScrollTo(cs.CurrentLine,   0)
				cs.Update()
			}
			return nil
		case tcell.KeyDown: // Handle the scroll down event
			if cs.CurrentLine < cs.MaxLines-1 {
				cs.CurrentLine++
				cs.TextView.ScrollTo(cs.CurrentLine,   0)
				cs.Update()
			}
			return nil
		default:
			return event
		}
	})
}
func AddCustomScrollbar(textView *tview.TextView, scrollbar *tview.TextView) {
	// Create a Flex layout to arrange the TextView and Scrollbar side by side
	flex := tview.NewFlex().
		AddItem(textView,  0,  1, false).
		AddItem(scrollbar,  1,  1, false)

	// Set the Flex layout as the root element of the application
	app.SetRoot(flex, true)
}
func main() {
	app := tview.NewApplication()
	// Erstellen Sie ein InputField für die Wildcard-Suche
	searchField := tview.NewInputField().
					SetLabel("Search: ").
					SetPlaceholder("Enter search term")
	outputView := tview.NewTextView().
					SetDynamicColors(true).
					SetRegions(true).
					SetChangedFunc(func() {app.Draw()})
		// Create a custom scrollbar for the TextView
		customScrollbar := NewCustomScrollbar(outputView, app) // Pass app as an argument

		// Add the custom scrollbar to the TextView
	AddCustomScrollbar(outputView, customScrollbar.Scrollbar)
	CommandButton1 := NewCommandButton("microk8s kubectl get nodes -A", outputView).Button
	CommandButton2 := NewCommandButton("microk8s kubectl get pods -A", outputView).Button
	// Erstellen Sie ein Flex-Layout, um das InputField und die Buttons nebeneinander anzuordnen
	headerFlex := tview.NewFlex().
		AddItem(CommandButton1,   0,   1, false).
		AddItem(CommandButton2,   0,   1, false).
		AddItem(searchField,   	  0,   1, false)
	// Erstellen Sie ein Grid-Layout
	grid := tview.NewGrid().
		SetRows(3,   0,   3).
		SetColumns(0,   0,   0).
		SetBorders(true)

	// Fügen Sie das Flex-Layout in das Grid-Feld ein, das den header repräsentiert
//	grid.AddItem(headerFlex,   0,   3,   1,   3,   0,   0, false)
grid.AddItem(headerFlex,   0,   0,   1,   3,   0,   0, false)
grid.AddItem(outputView,   1,   0,   1,   1,   0,   0, false)

// Setzen Sie das Grid-Layout als Root-Element der Anwendung
	if err := app.SetRoot(grid, true).EnableMouse(true).Run(); err != nil {
		panic(err)
	}
}
