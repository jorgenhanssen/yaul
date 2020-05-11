package main

import (
	"os"

	"github.com/jorgenhanssen/a-machine/local/tape"
)

var (
	programCounter int

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

	// read program file and parse instructions
	fileData, err := ReadFile(os.Args[1])
	ensure(err)
	program, err := extractInstructions(fileData)
	ensure(err)

	// Insert instructions
	for i, instruction := range program {
		ensure(instructions.Write(i, instruction))
	}

	// run program
	for ; ; programCounter++ {
		i, err := instructions.Read(programCounter)
		if err == tape.ErrNilValue {
			break // no more instructions
		}
		ensure(err)

		// programCounter++
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
