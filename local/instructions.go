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

// jumpTo is a helper that ticks the program counter
// to the correct address for when the next instruction is run.
// we need to subtract 2 from the address:
// 	1 - the addresses are shifted 1 down from their reference in a text file (line numbers start a 1)
//  2 - the program counter is incremented by 1 after the jumpTo command is run (before next read)
func jumpTo(reg int) {
	programCursor = reg - 2
}

func RunInstruction(i *Instruction) error {
	switch i.command {
	case iTerminate:
		{
			logger.Silent("Terminate")
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

	logger.Silent(fmt.Sprintf("Set value '%d' into #%d", val.data, reg))
	return values.Write(reg, val.data)
}
func In(address *Param) error {
	fmt.Print("Input: ")
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

	logger.Silent(fmt.Sprintf("Input value '%d' into #%d", num, reg))
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

	logger.Silent(fmt.Sprintf("Output from #%d ('%d')", reg, val))
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

	logger.Silent(fmt.Sprintf("Move #%d ('%d') into #%d", regA, valueA, regB))
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

	logger.Silent(fmt.Sprintf("Add #%d ('%d') and #%d ('%d') into #%d ('%d')", regA, valueA, regB, valueB, regDest, valueA+valueB))
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
	logger.Silent(fmt.Sprintf("Subtract #%d ('%d') from #%d ('%d') into #%d ('%d')", regB, valueB, regA, valueA, regDest, valueA-valueB))
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
	logger.Silent(fmt.Sprintf("Multiply #%d ('%d') and #%d ('%d') into #%d ('%d')", regA, valueA, regB, valueB, regDest, valueA*valueB))
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
	logger.Silent(fmt.Sprintf("Divide #%d ('%d') by #%d ('%d') into #%d ('%d')", regA, valueA, regB, valueB, regDest, valueA/valueB))
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
	logger.Silent(fmt.Sprintf("Modulo #%d ('%d') by #%d ('%d') into #%d ('%d')", regA, valueA, regB, valueB, regDest, valueA%valueB))
	return values.Write(regDest, valueA%valueB)
}

// Conditions
func Jmp(address *Param) error {
	reg, err := safeReadAddress(address)
	if err != nil {
		return err
	}
	jumpTo(reg)
	logger.Silent(fmt.Sprintf("Jump to l%d", reg))
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
		logger.Silent(fmt.Sprintf("Jump to l%d [#%d ('%d') > #%d ('%d')]", reg, regA, valueA, regB, valueB))
		jumpTo(reg)
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
		logger.Silent(fmt.Sprintf("Jump to l%d [#%d ('%d') == #%d ('%d')]", reg, regA, valueA, regB, valueB))
		jumpTo(reg)
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
		logger.Silent(fmt.Sprintf("Jump to l%d [#%d ('%d') < #%d ('%d')]", reg, regA, valueA, regB, valueB))
		jumpTo(reg)
	}
	return nil
}
