package tape

type Instance struct {
	memory []interface{}
}

func New(size int) *Instance {
	return &Instance{
		memory: make([]interface{}, size),
	}
}

func (i *Instance) minAvailableMemoryAddresses() int {
	return 0
}

func (i *Instance) maxAvailableMemoryAddresses() int {
	return len(i.memory)
}
