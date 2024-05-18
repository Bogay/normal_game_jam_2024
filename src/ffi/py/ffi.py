def py_create_bullet(data:bytes)->bytes:
    speed = int.from_bytes(data[8:8+4], 'little')
    print("data: ", data.hex())
    res= data[:8] + \
           int(speed * 0.75).to_bytes(4, 'little') + \
           data[12:-5] + \
           b"\x01" + \
           data[-4:]
    print("res:  ", res.hex())
    return res