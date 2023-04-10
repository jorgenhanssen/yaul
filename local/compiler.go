package main

import (
	"fmt"
	"io/ioutil"
	"math"
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
	for _, line := range lines {
		if instruction, err := p.parseInstruction(line); err == nil {
			instructions = append(instructions, instruction)
		} else {
			return nil, p.lineError(err, line)
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

	instruction := strings.ToUpper(stringData[0])

	iID := iMap[instruction]
	if iID == 0 {
		return nil, fmt.Errorf("unknown instruction '%s'", instruction)
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

		// jump instructions have a label as their first parameter
		if isJumpInstruction && i == 0 {
			instructionIndex, ok := p.labels[str]
			if !ok {
				return nil, fmt.Errorf("unknown label '%s'", str)
			}

			params = append(params, &Param{
				isReference: false,
				data:        instructionIndex,
			})
			continue
		}

		// jump instructions have a label as their third parameter
		if isJumpCompareInstruction && i == 2 {
			instructionIndex, ok := p.labels[str]
			if !ok {
				return nil, fmt.Errorf("unknown label '%s'", str)
			}

			params = append(params, &Param{
				isReference: false,
				data:        instructionIndex,
			})
			continue
		}

		num, err := strconv.Atoi(str)
		if err != nil {
			return nil, fmt.Errorf("Instruction %s got non-numeric parameter[#%d] '%s' ", instruction, i+1, str)
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

func (c *Compiler) lineError(err error, line string) error {
	e := fmt.Sprintf("Error: %v", err)
	l := fmt.Sprintf("Line: %s", line)
	bars := strings.Repeat("=", int(math.Max(float64(len(e)), float64(len(l)))))

	return fmt.Errorf("\n\n%s\n%s\n\n%s\n%s", bars, e, l, bars)
}

// lineIsNonFunctional returns true if the line is a comment or empty
func lineIsNonFunctional(line string) bool {
	if line == "" {
		return true
	}

	for i, char := range line {
		if !unicode.IsSpace(char) {
			if i == len(line)-1 {
				return true // End of line, ergo empty
			}

			// Not empty, let's check if it's a comment
			if char == '/' && line[i+1] == '/' {
				return true
			}

			break
		}
	}

	return false
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
