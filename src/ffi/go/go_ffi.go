package main

import (
	"C"
	"encoding/binary"
)
import (
	"bytes"
)

type Bullet struct {
	Damage          uint32
	Count           uint32
	Speed           uint32
	Hp              uint32
	R               uint8
	G               uint8
	B               uint8
	A               uint8
	Damage_by_frame uint8
	Mp_cost         uint32
}

//export go_ffi
func go_ffi(bullet []byte) {
	obj := Bullet{}
	err := binary.Read(bytes.NewBuffer(bullet), binary.LittleEndian, &obj)
	if err != nil {
		panic(err)
	}
	obj.Hp += obj.Count
	buf := &bytes.Buffer{}
	binary.Write(buf, binary.LittleEndian, obj)
	buf.Write(bullet)
}

func main() {
	// Need a main function to make CGO compile package as C shared library
}
