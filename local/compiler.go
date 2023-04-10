package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
	"time"
	"unicode"

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

// NewCompiler creates a new compiler instance
func NewCompiler(logger *logging.Instance) *Compiler {
	return &Compiler{
		logger: logger,
		labels: map[string]int{},
	}
}

// Compile parses the source code and returns a list of instructions
func (p *Compiler) Compile(fileData string) (Instructions, error) {
	var instructions Instructions

	compileStart := time.Now()

	lines := p.resolveLines(fileData)
	for i, line := range lines {
		if instruction, err := p.parseInstruction(line); err == nil {
			instructions = append(instructions, instruction)
		} else {
			return nil, fmt.Errorf("Error at line %d: %v", i+1, err)
		}
	}

	compileEnd := time.Now()
	logger.Print(fmt.Sprintf("Compiled in %v", compileEnd.Sub(compileStart)))

	return instructions, nil
}

// resolveLines removes comments and labels from the source code
// and resolves labels to their corresponding instruction index
func (p *Compiler) resolveLines(data string) []string {
	resolved := []string{}

	for _, line := range strings.Split(data, "\n") {
		if lineIsNonFunctional(line) {
			continue
		}
		if lineIsLabel(line) {
			p.labels[strings.Split(line, ":")[0]] = len(resolved) + 1
			continue
		}

		resolved = append(resolved, line)
	}

	return resolved
}

// parseInstruction parses a single line of source code and returns an instruction
func (p *Compiler) parseInstruction(line string) (*Instruction, error) {
	stringData := strings.Split(line, " ")

	iID := iMap[strings.ToUpper(stringData[0])]
	if iID == 0 {
		return nil, fmt.Errorf("unknown instruction '%s'", line)
	}

	isJumpInstruction := iID == iJump
	isJumpCompareInstruction := iID >= iJumpGreaterThan && iID <= iJumpLessThan

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

// lineIsNonFunctional returns true if the line is a comment or empty
func lineIsNonFunctional(line string) bool {
	// Check if the line is a comment
	if len(line) >= 2 && line[0] == '/' && line[1] == '/' {
		return true
	}

	// Check if the line is empty
	for _, c := range line {
		if !unicode.IsSpace(c) {
			return false
		}
	}

	return true
}

// lineIsLabel returns true if the line is a label
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

// ReadFile reads a file and returns its contents as a string
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
