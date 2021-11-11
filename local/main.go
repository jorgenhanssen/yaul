package main

import (
	"fmt"
	"os"

	"github.com/jorgenhanssen/a-machine/local/logging"
	"github.com/jorgenhanssen/a-machine/local/tape"
)

var (
	programCursor int

	instructions *tape.Instance
	values       *tape.Instance
	logger       *logging.Instance
)

const maxInstructions = 256
const maxHeapSize = 256

func main() {
	logger = logging.New()
	defer logger.ToFile()

	instructions = tape.New(maxInstructions)
	values = tape.New(maxHeapSize)

	if len(os.Args) == 1 {
		logger.Print("No file specified")
		return
	}
	if len(os.Args) > 2 {
		logger.Print("To many parameters. Specify the source file only.")
		return
	}

	// read program file and parse instructions
	fileData, err := ReadFile(os.Args[1])
	ensure(err)

	logger.LoadProgram(fileData)
	program, err := extractInstructions(fileData)
	ensure(err)

	// Insert instructions
	for i, instruction := range program {
		ensure(instructions.Write(i, instruction))
	}

	// run program
	for ; ; programCursor++ {
		i, err := instructions.Read(programCursor)
		if err == tape.ErrNilValue {
			break // no more instructions
		}
		ensure(err)

		err = RunInstruction(i.(*Instruction))
		if err == ErrTerminate {
			break // program invoked exit
		}
		ensure(err)
	}
}

func ensure(err error) {
	if err != nil {
		logger.Silent(fmt.Sprint("[FATAL] ", err))
		panic(err)
	}
}
