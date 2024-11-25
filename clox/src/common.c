#include "common.h"
#include "chunk.h"
#include "debug.h"
#include "vm.h"

#define DEBUG_TRACE_EXECUTION

int main(int argc, const char *argv[])
{
  Chunk chunk;
  initChunk(&chunk);
  writeChunk(&chunk, OP_RETURN, 123);

  int constant = addConstant(&chunk, 1.2);
  writeChunk(&chunk, OP_CONSTANT, 123);
  writeChunk(&chunk, constant, 123);
  
  disassembleChunk(&chunk, "test chunk");
  freeChunk(&chunk);
  return 0;
}