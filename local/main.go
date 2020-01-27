package main

import (
	"os"

	"github.com/jorgenhanssen/a-machine/local/tape"
)

var (
	instructionPosition int

	instructions *tape.Instance
	values       *tape.Instance
)

func main() {
	instructions = tape.New(256)
	values = tape.New(256)

	if len(os.Args) == 1 {
		println("No file specified.")
		return
	}
	if len(os.Args) > 2 {
		println("To many parameters. Specify the source file only.")
		return
	}

	fileData, err := ReadFile(os.Args[1])
	ensure(err)

	parsedInstructions, err := ParseFile(fileData)
	ensure(err)

	for i, instruction := range parsedInstructions {
		ensure(instructions.Write(i, instruction))
	}

	// run program
	for {
		i, err := instructions.Read(instructionPosition)
		ensure(err)
		if i == nil {
			// no more instructions
			break
		}

		err = RunInstruction(i.(*Instruction))
		if err == ErrTerminate {
			break // program invoked exit
		}
		ensure(err)
	}
}

func ensure(err error) {
	if err != nil {
		panic(err)
	}
}
