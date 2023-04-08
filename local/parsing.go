package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type Parser struct {
	jumpsToResolve map[int]bool
}

type Param struct {
	isReference bool
	data        int
}

type Instruction struct {
	command int
	params  []*Param
}
type Instructions []*Instruction

func ReadFile(filename string) (string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return "", err
	}

	defer file.Close()

	if b, err := ioutil.ReadAll(file); err != nil {
		return "", err
	} else {
		return string(b), nil
	}
}

func NewParser() *Parser {
	return &Parser{
		jumpsToResolve: map[int]bool{},
	}
}

func (p *Parser) extractInstructions(fileData string) (Instructions, error) {
	lines := strings.Split(fileData, "\n")

	var instructions Instructions
	for i, line := range lines {
		instruction, err := p.parseInstruction(line)
		if err != nil {
			return nil, fmt.Errorf("Error at line %d: %v", i+1, err)
		}

		if instruction != nil {
			// Instruction may be nil if line is a comment or empty line
			instructions = append(instructions, instruction)
		}
	}

	// resolve jumps
	lineToInstructionIndex := map[int]int{} // code line => instruction index
	offset := 0
	for i, line := range lines {
		if lineIsNonFunctional(line) {
			offset++
			continue
		}

		lineNumber := i + 1
		if p.jumpsToResolve[lineNumber] {
			lineToInstructionIndex[lineNumber] = lineNumber - offset
		}
	}

	// adjust jump addresses from code line to instruction index
	for _, in := range instructions {
		if isOneOf(in.command, iJumpGreaterThan, iJumpEqual, iJumpLessThan) {
			in.params[2].data = lineToInstructionIndex[in.params[2].data]
		} else if isOneOf(in.command, iJump) {
			in.params[0].data = lineToInstructionIndex[in.params[0].data]
		}
	}

	return instructions, nil
}

func (p *Parser) parseInstruction(iLine string) (*Instruction, error) {
	if lineIsNonFunctional(iLine) {
		return nil, nil
	}

	stringData := strings.Split(iLine, " ")

	iID := iMap[strings.ToUpper(stringData[0])]
	if iID == 0 {
		return nil, fmt.Errorf("unknown instruction '%s'", iLine)
	}

	stringData = stringData[1:]

	var params []*Param
	for _, str := range stringData {
		if str == "" {
			continue // ignore multiple whitespaces
		}
		if str == "//" {
			break // ignore comments
		}

		isReference := false
		if strings.HasPrefix(str, "&") {
			str = strings.TrimPrefix(str, "&")
			isReference = true
		} else {
			// values can have '' for better readability
			str = strings.TrimPrefix(str, "'")
			str = strings.TrimSuffix(str, "'")
		}

		num, err := strconv.Atoi(str)
		if err != nil {
			return nil, err
		}

		params = append(params, &Param{
			isReference: isReference,
			data:        num,
		})
	}

	// if instruction is a jump, we need to adjust the address for
	// empty lines and single-line comments
	if isOneOf(iID, iJumpGreaterThan, iJumpEqual, iJumpLessThan) {
		p.jumpsToResolve[params[2].data] = true
	} else if isOneOf(iID, iJump) {
		p.jumpsToResolve[params[0].data] = true
	}

	return &Instruction{
		command: iID,
		params:  params,
	}, nil
}

func isOneOf(value int, values ...int) bool {
	for _, v := range values {
		if value == v {
			return true
		}
	}
	return false
}

func safeInterfaceToInt(value interface{}) (int, error) {
	val, ok := value.(int)
	if !ok {
		return 0, fmt.Errorf("unable to cast '%v' to int", val)
	}
	return val, nil
}

func safeReadInt(address int) (int, error) {
	val, err := values.Read(address)
	if err != nil {
		return 0, err
	}
	safeVal, err := safeInterfaceToInt(val)
	if err != nil {
		return 0, err
	}
	return safeVal, nil
}

// used to return an address or a reference
func safeReadAddress(address *Param) (int, error) {
	reg := address.data
	if address.isReference {
		var err error
		reg, err = safeReadInt(address.data)
		if err != nil {
			return 0, err
		}
	}
	return reg, nil
}

// if empty or comment (starts with //), return true
func lineIsNonFunctional(line string) bool {
	return strings.TrimSpace(line) == "" || strings.HasPrefix(line, "//")
}
