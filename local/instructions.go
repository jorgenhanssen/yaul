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
	ErrTerminate = errors.New("Program end.")
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

func RunInstruction(i *Instruction) error {
	instructionPosition++

	switch i.command {
	case iTerminate:
		{
			return ErrTerminate
		}
	case iSet:
		{
			return Set(i.data[0], i.data[1])
		}
	case iInput:
		{
			return In(i.data[0])
		}
	case iOutput:
		{
			return Out(i.data[0])
		}
	case iAdd:
		{
			return Add(i.data[0], i.data[1], i.data[2])
		}
	case iSubtract:
		{
			return Sub(i.data[0], i.data[1], i.data[2])
		}
	case iMultiply:
		{
			return Mul(i.data[0], i.data[1], i.data[2])
		}
	case iDivide:
		{
			return Div(i.data[0], i.data[1], i.data[2])
		}
	case iModulo:
		{
			return Mod(i.data[0], i.data[1], i.data[2])
		}
	case iJump:
		{
			return Jmp(i.data[0])
		}
	case iJumpGreaterThan:
		{
			return Jgt(i.data[0], i.data[1], i.data[2])
		}
	case iJumpEqual:
		{
			return Jeq(i.data[0], i.data[1], i.data[2])
		}
	case iJumpLessThan:
		{
			return Jlt(i.data[0], i.data[1], i.data[2])
		}
	case iMove:
		{
			return Mov(i.data[0], i.data[1])
		}
	default:
		{
			return fmt.Errorf("Command not implemented: %d", i.command)
		}
	}
}

func Set(val, address int) error {
	return values.Write(address, val)
}
func In(address int) error {
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

	return values.Write(address, num)
}
func Out(address int) error {
	val, err := safeReadInt(address)
	if err != nil {
		return err
	}
	println(val)
	return nil
}
func Mov(a, b int) error {
	valueA, err := safeReadInt(a)
	if err != nil {
		return err
	}
	return values.Write(b, valueA)
}

// Arithmetic
func Add(a, b, dest int) error {
	valueA, err := safeReadInt(a)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(b)
	if err != nil {
		return err
	}

	return values.Write(dest, valueA+valueB)
}

func Sub(a, b, dest int) error {
	valueA, err := safeReadInt(a)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(b)
	if err != nil {
		return err
	}
	return values.Write(dest, valueA-valueB)
}

func Mul(a, b, dest int) error {
	valueA, err := safeReadInt(a)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(b)
	if err != nil {
		return err
	}
	return values.Write(dest, valueA*valueB)
}

func Div(a, b, dest int) error {
	valueA, err := safeReadInt(a)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(b)
	if err != nil {
		return err
	}
	return values.Write(dest, valueA/valueB)
}

func Mod(a, b, dest int) error {
	valueA, err := safeReadInt(a)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(b)
	if err != nil {
		return err
	}
	return values.Write(dest, valueA%valueB)
}

// Conditions
func Jmp(address int) error {
	instructionPosition = address - 1
	return nil
}
func Jgt(a, b, address int) error {
	valueA, err := safeReadInt(a)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(b)
	if err != nil {
		return err
	}
	if valueA > valueB {
		instructionPosition = address - 1
	}
	return nil
}
func Jeq(a, b, address int) error {
	valueA, err := safeReadInt(a)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(b)
	if err != nil {
		return err
	}
	if valueA == valueB {
		instructionPosition = address - 1
	}
	return nil
}
func Jlt(a, b, address int) error {
	valueA, err := safeReadInt(a)
	if err != nil {
		return err
	}
	valueB, err := safeReadInt(b)
	if err != nil {
		return err
	}
	if valueA < valueB {
		instructionPosition = address - 1
	}
	return nil
}
