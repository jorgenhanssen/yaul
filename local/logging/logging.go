package logging

import (
	"fmt"
	"os"
	"time"
)

type LogEntry struct {
	text string
	time time.Time
}

type Instance struct {
	logs    []LogEntry
	program string
}

func New() *Instance {
	return &Instance{
		logs: []LogEntry{},
	}
}

func (i *Instance) LoadProgram(program string) {
	i.program = program
}

func (i *Instance) Print(text string) {
	i.Silent(text)
	fmt.Println(text)
}

func (i *Instance) Silent(text string) {
	i.logs = append(i.logs, LogEntry{
		text: text,
		time: time.Now(),
	})
}

func (i *Instance) ToFile() {
	fmt.Println("Writing logs...")
	data := i.program + "\n\n"
	for _, log := range i.logs {
		data += log.time.Format("2006/01/02 - 15:04:05.999999:  \t") + log.text + "\n"
	}

	d1 := []byte(data)
	if err := os.WriteFile("log", d1, 0644); err != nil {
		fmt.Println(err)
	}
}
