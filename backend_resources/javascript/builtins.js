module.exports = {
  '::i8': {
    size_of: function (input) { return 1 },
    serialize: function (input, buffer, offset) {
      buffer.writeInt8(input, offset)
      return offset + 1
    },
    deserialize: function (buffer, offset) {
      return {value: buffer.readInt8(offset), size: offset + 1}
    }
  },
  '::u8': {
    size_of: function (input) { return 1 },
    serialize: function (input, buffer, offset) {
      buffer.writeInt8(input, offset)
      return offset + 1
    },
    deserialize: function (buffer, offset) {
      return {value: buffer.readInt8(offset), size: offset + 1}
    }
  },
  '::i16': {
    size_of: function (input) { return 2 },
    serialize: function (input, buffer, offset) {
      buffer.writeInt16BE(input, offset)
      return offset + 2
    },
    deserialize: function (buffer, offset) {
      return {value: buffer.readInt16BE(offset), size: offset + 2}
    }
  },
  '::u16': {
    size_of: function (input) { return 2 },
    serialize: function (input, buffer, offset) {
      buffer.writeUInt16BE(input, offset)
      return offset + 2
    },
    deserialize: function (buffer, offset) {
      return {value: buffer.readUInt16BE(offset), size: offset + 2}
    }
  },
  '::i32': {
    size_of: function (input) { return 4 },
    serialize: function (input, buffer, offset) {
      buffer.writeInt32BE(input, offset)
      return offset + 4
    },
    deserialize: function (buffer, offset) {
      return {value: buffer.readInt32BE(offset), size: offset + 4}
    }
  },
  '::u32': {
    size_of: function (input) { return 4 },
    serialize: function (input, buffer, offset) {
      buffer.writeUInt32BE(input, offset)
      return offset + 4
    },
    deserialize: function (buffer, offset) {
      return {value: buffer.readUInt32BE(offset), size: offset + 4}
    }
  },
  '::i64': {
    size_of: function (input) { return 8 },
    serialize: function (input, buffer, offset) {
      buffer.writeInt64BE(input, offset)
      return offset + 8
    },
    deserialize: function (buffer, offset) {
      return {value: buffer.readInt64BE(offset), size: offset + 8}
    }
  },
  '::u64': {
    size_of: function (input) { return 8 },
    serialize: function (input, buffer, offset) {
      buffer.writeUInt64BE(input, offset)
      return offset + 8
    },
    deserialize: function (buffer, offset) {
      return {value: buffer.readUInt64BE(offset), size: offset + 8}
    }
  },
  '::varint': {
    size_of: function (value) {
      let cursor = 0
      while (value & ~0x7F) {
        value >>>= 7
        cursor++
      }
      return cursor + 1
    }
  },
  '::sized_string': {
    size_of: function (input) { return Buffer.byteLength(input, 'utf8') },
    serialize: function (input, buffer, offset) {
      return offset + buffer.write(input, offset)
    },
    deserialize: function (buffer, offset, size) {
      return {value: buffer.toString('utf8', offset, offset + size), size: offset + size}
    }
  },
  '::unit': {
    size_of: function (input) { return 0 },
    serialize: function (input, buffer, offset) { return offset },
    deserialize: function (buffer, offset) { return {value: null, size: offset} }
  }
}
