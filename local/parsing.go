package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"

	"github.com/jorgenhanssen/a-machine/local/logging"
)

type Parser struct {
	logger            *logging.Instance
	instructionCursor int
	labels            map[string]int
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

func NewParser(logger *logging.Instance) *Parser {
	return &Parser{
		logger: logger,
		labels: map[string]int{},
	}
}

func (p *Parser) extractInstructions(fileData string) (Instructions, error) {
	lines := strings.Split(fileData, "\n")

	p.resolveLabels(&lines)

	var instructions Instructions
	for i, line := range lines {
		instruction, err := p.parseInstruction(line)
		if err != nil {
			return nil, fmt.Errorf("Error at line %d: %v", i+1, err)
		}

		// Instruction may be nil if line is a comment or empty line
		if instruction != nil {
			instructions = append(instructions, instruction)
			p.instructionCursor++
		}
	}

	return instructions, nil
}

func (p *Parser) resolveLabels(lines *[]string) {
	offset := 0
	for i, line := range *lines {
		if lineIsNonFunctional(line) {
			offset++
			continue
		}

		tokens := strings.Split(line, " ")
		if strings.HasSuffix(tokens[0], ":") {
			p.labels[strings.TrimSuffix(tokens[0], ":")] = i - offset + 1
			offset++
		}
	}
}

func (p *Parser) parseInstruction(iLine string) (*Instruction, error) {
	if lineIsNonFunctional(iLine) {
		return nil, nil
	}

	stringData := strings.Split(iLine, " ")

	// check if label
	if strings.HasSuffix(stringData[0], ":") {
		return nil, nil
	}

	iID := iMap[strings.ToUpper(stringData[0])]
	if iID == 0 {
		return nil, fmt.Errorf("unknown instruction '%s'", iLine)
	}

	isJumpInstruction := iID == iJump
	isJumpCompareInstruction := isOneOf(iID, iJumpGreaterThan, iJumpEqual, iJumpLessThan)

	var params []*Param
	for i, str := range stringData[1:] {
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

		if isJumpInstruction && i == 0 {
			// jump instructions have a label as their first parameter
			params = append(params, &Param{
				isReference: false,
				data:        p.labels[str],
			})
			continue
		}

		if isJumpCompareInstruction && i == 2 {
			// jump instructions have a label as their third parameter
			params = append(params, &Param{
				isReference: false,
				data:        p.labels[str],
			})
			continue
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
