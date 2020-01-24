package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type Instruction struct {
	command int
	data    []int
}
type Instructions []*Instruction

func ReadFile(filename string) (string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return "", err
	}
	defer file.Close()

	b, err := ioutil.ReadAll(file)
	if err != nil {
		return "", err
	}

	return string(b), nil
}

func ParseFile(fileData string) (Instructions, error) {
	lines := strings.Split(fileData, "\n")

	var instructions Instructions
	for i, line := range lines {
		instruction, err := parseInstruction(line)
		if err != nil {
			return nil, fmt.Errorf("Error at line %d: %v", i+1, err)
		}
		instructions = append(instructions, instruction)
	}

	return instructions, nil
}

func parseInstruction(iLine string) (*Instruction, error) {
	stringData := strings.Split(iLine, " ")
	if stringData[0] == "" {
		return nil, errors.New("empty line not allowed.")
	}

	iID := iMap[strings.ToUpper(stringData[0])]
	if iID == 0 {
		return nil, fmt.Errorf("unknown instruction '%s'", iLine)
	}

	stringData = stringData[1:]

	var data []int
	for _, str := range stringData {
		if str == "" {
			continue // ignore multiple whitespaces
		}
		if str == "//" {
			break // ignore comments
		}

		// values can have '' for easy reading
		str = strings.TrimPrefix(str, "'")
		str = strings.TrimSuffix(str, "'")

		num, err := strconv.Atoi(str)
		if err != nil {
			return nil, err
		}

		data = append(data, num)
	}

	return &Instruction{
		command: iID,
		data:    data,
	}, nil
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
