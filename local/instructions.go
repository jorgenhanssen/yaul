package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var (
	ErrTerminate                = errors.New("Program end.")
	ErrValuesCannotBeReferences = errors.New("Values cannot be references")
)

const (
	iSet             = 1
	iInput           = 2
	iOutput          = 3
	iAdd             = 4
	iSubtract        = 5
	iMultiply        = 6
	iDivide          = 7
	iModulo          = 8
	iJump            = 10
	iJumpGreaterThan = 11
	iJumpEqual       = 12
	iJumpLessThan    = 13
	iMove            = 14
	iTerminate       = 15
)

// iMap (instruction map) is used for mapping instructions from files
var iMap = map[string]int{
	"SET":  iSet,
	"IN":   iInput,
	"OUT":  iOutput,
	"ADD":  iAdd,
	"SUB":  iSubtract,
	"MUL":  iMultiply,
	"DIV":  iDivide,
	"MOD":  iModulo,
	"JMP":  iJump,
	"JGT":  iJumpGreaterThan,
	"JEQ":  iJumpEqual,
	"JLT":  iJumpLessThan,
	"MOV":  iMove,
	"TERM": iTerminate,
}

// jumpToLine is a helper that ticks the program counter
// to the correct address for when the next instruction is run.
// we need to subtract 2 from the address:
// 	1 - the addresses are shifted 1 down from their reference in a text file (line numbers start a 1)
//  2 - the program counter is incremented by 1 after the jumpToLine command is run (before next read)
func jumpToLine(line int) {
	programCounter = line - 2
}

func RunInstruction(i *Instruction) error {
	switch i.command {
	case iTerminate:
		{
			return ErrTerminate
		}
	case iSet:
		{
			return Set(i.params[0], i.params[1])
		}
	case iInput:
		{
			return In(i.params[0])
		}
	case iOutput:
		{
			return Out(i.params[0])
		}
	case iAdd:
		{
			return Add(i.params[0], i.params[1], i.params[2])
		}
	case iSubtract:
		{
			return Sub(i.params[0], i.params[1], i.params[2])
		}
	case iMultiply:
		{
			return Mul(i.params[0], i.params[1], i.params[2])
		}
	case iDivide:
		{
			return Div(i.params[0], i.params[1], i.params[2])
		}
	case iModulo:
		{
			return Mod(i.params[0], i.params[1], i.params[2])
		}
	case iJump:
		{
			return Jmp(i.params[0])
		}
	case iJumpGreaterThan:
		{
			return Jgt(i.params[0], i.params[1], i.params[2])
		}
	case iJumpEqual:
		{
			return Jeq(i.params[0], i.params[1], i.params[2])
		}
	case iJumpLessThan:
		{
			return Jlt(i.params[0], i.params[1], i.params[2])
		}
	case iMove:
		{
			return Mov(i.params[0], i.params[1])
		}
	default:
		{
			return fmt.Errorf("Command not implemented: %d", i.command)
		}
	}
}

func Set(val, address *Param) error {
	if val.isReference {
		return ErrValuesCannotBeReferences
	}
	reg, err := safeReadAddress(address)
	if err != nil {
		return err
	}
	return values.Write(reg, val.data)
}
func In(address *Param) error {
	reader := bufio.NewReader(os.Stdin)
	text, err := reader.ReadString('\n')
	if err != nil {
		return err
	}

	text = strings.Replace(text, "\n", "", 1)

	num, err := strconv.Atoi(text)
	if err != nil {
		return err
	}

	reg, err := safeReadAddress(address)
	if err != nil {
		return err
	}
	return values.Write(reg, num)
}
func Out(address *Param) error {
	reg, err := safeReadAddress(address)
	if err != nil {
		return err
	}

	val, err := safeReadInt(reg)
	if err != nil {
		return err
	}
	println(val)
	return nil
}
func Mov(a, b *Param) error {
	regA, err := safeReadAddress(a)
	if err != nil {
		return err
	}
	valueA, err := safeReadInt(regA)
	if err != nil {
		return err
	}

	regB, err := safeReadAddress(b)
	if err != nil {
		return err
	}

	return values.Write(regB, valueA)
}

// Arithmetic
func Add(a, b, dest *Param) error {
	regA, err := safeReadAddress(a)
	if err != nil {
		return err
	}
	valueA, err := safeReadInt(regA)
	if err != nil {
		return err
	}

	regB, err := safeReadAddress(b)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(regB)
	if err != nil {
		return err
	}

	regDest, err := safeReadAddress(dest)
	if err != nil {
		return err
	}
	return values.Write(regDest, valueA+valueB)
}

func Sub(a, b, dest *Param) error {
	regA, err := safeReadAddress(a)
	if err != nil {
		return err
	}
	valueA, err := safeReadInt(regA)
	if err != nil {
		return err
	}

	regB, err := safeReadAddress(b)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(regB)
	if err != nil {
		return err
	}

	regDest, err := safeReadAddress(dest)
	if err != nil {
		return err
	}
	return values.Write(regDest, valueA-valueB)
}

func Mul(a, b, dest *Param) error {
	regA, err := safeReadAddress(a)
	if err != nil {
		return err
	}
	valueA, err := safeReadInt(regA)
	if err != nil {
		return err
	}

	regB, err := safeReadAddress(b)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(regB)
	if err != nil {
		return err
	}

	regDest, err := safeReadAddress(dest)
	if err != nil {
		return err
	}
	return values.Write(regDest, valueA*valueB)
}

func Div(a, b, dest *Param) error {
	regA, err := safeReadAddress(a)
	if err != nil {
		return err
	}
	valueA, err := safeReadInt(regA)
	if err != nil {
		return err
	}

	regB, err := safeReadAddress(b)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(regB)
	if err != nil {
		return err
	}

	regDest, err := safeReadAddress(dest)
	if err != nil {
		return err
	}
	return values.Write(regDest, valueA/valueB)
}

func Mod(a, b, dest *Param) error {
	regA, err := safeReadAddress(a)
	if err != nil {
		return err
	}
	valueA, err := safeReadInt(regA)
	if err != nil {
		return err
	}

	regB, err := safeReadAddress(b)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(regB)
	if err != nil {
		return err
	}

	regDest, err := safeReadAddress(dest)
	if err != nil {
		return err
	}
	return values.Write(regDest, valueA%valueB)
}

// Conditions
func Jmp(address *Param) error {
	reg, err := safeReadAddress(address)
	if err != nil {
		return err
	}
	jumpToLine(reg)
	return nil
}
func Jgt(a, b, address *Param) error {
	regA, err := safeReadAddress(a)
	if err != nil {
		return err
	}
	valueA, err := safeReadInt(regA)
	if err != nil {
		return err
	}

	regB, err := safeReadAddress(b)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(regB)
	if err != nil {
		return err
	}

	reg, err := safeReadAddress(address)
	if err != nil {
		return err
	}
	if valueA > valueB {
		jumpToLine(reg)
	}
	return nil
}
func Jeq(a, b, address *Param) error {
	regA, err := safeReadAddress(a)
	if err != nil {
		return err
	}
	valueA, err := safeReadInt(regA)
	if err != nil {
		return err
	}

	regB, err := safeReadAddress(b)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(regB)
	if err != nil {
		return err
	}

	reg, err := safeReadAddress(address)
	if err != nil {
		return err
	}
	if valueA == valueB {
		jumpToLine(reg)
	}
	return nil
}
func Jlt(a, b, address *Param) error {
	regA, err := safeReadAddress(a)
	if err != nil {
		return err
	}
	valueA, err := safeReadInt(regA)
	if err != nil {
		return err
	}

	regB, err := safeReadAddress(b)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(regB)
	if err != nil {
		return err
	}

	reg, err := safeReadAddress(address)
	if err != nil {
		return err
	}
	if valueA < valueB {
		jumpToLine(reg)
	}
	return nil
}
