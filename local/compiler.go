package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"

	"github.com/jorgenhanssen/a-machine/local/logging"
)

type Compiler struct {
	logger *logging.Instance
	labels map[string]int
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

func NewCompiler(logger *logging.Instance) *Compiler {
	return &Compiler{
		logger: logger,
		labels: map[string]int{},
	}
}

func (p *Compiler) Compile(fileData string) (Instructions, error) {
	lines := strings.Split(fileData, "\n")

	p.resolveLabels(&lines)

	var instructions Instructions
	for i, line := range lines {
		if lineIsNonFunctional(line) || lineIsLabel(line) {
			continue
		}

		if instruction, err := p.parseInstruction(line); err == nil {
			instructions = append(instructions, instruction)
		} else {
			return nil, fmt.Errorf("Error at line %d: %v", i+1, err)
		}
	}

	return instructions, nil
}

func (p *Compiler) resolveLabels(lines *[]string) {
	offset := 0
	for i, line := range *lines {
		if lineIsNonFunctional(line) {
			offset++
			continue
		}

		if lineIsLabel(line) {
			p.labels[strings.Split(line, ":")[0]] = i - offset + 1
			offset++
		}
	}
}

func (p *Compiler) parseInstruction(iLine string) (*Instruction, error) {
	stringData := strings.Split(iLine, " ")

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

func lineIsNonFunctional(line string) bool {
	return strings.HasPrefix(line, "//") || strings.TrimSpace(line) == ""
}

func lineIsLabel(line string) bool {
	for _, c := range line {
		if c == ' ' {
			return false
		}
		if c == ':' {
			return true
		}
	}
	return false
}
