# 定义编译器
CC = gcc

# 定义编译选项，例如 -g 用于调试，-Wall 用于显示所有警告
CFLAGS = -g -Wall -Iinclude

# 定义链接选项
LDFLAGS = 

# 定义源文件目录
SRC_DIR = src
# 定义二进制文件目录
BIN_DIR = bin
# 定义头文件目录
INCLUDE_DIR = include

# 定义源文件
SRC = $(wildcard $(SRC_DIR)/*.c)
# 定义目标文件，将源文件的 .c 后缀替换为 .o
OBJ = $(patsubst $(SRC_DIR)/%.c, $(BIN_DIR)/%.o, $(SRC))

# 定义最终的可执行文件名
TARGET = out.exe

# 默认目标
all: $(BIN_DIR)/$(TARGET)

# 链接目标
$(BIN_DIR)/$(TARGET): $(OBJ)
	$(CC) $(LDFLAGS) $^ -o $@

# 编译源文件
$(BIN_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

# 清理编译生成的文件
clean:
	rm -f $(BIN_DIR)/*.o $(BIN_DIR)/$(TARGET)

run: $(BIN_DIR)/$(TARGET)
	./$(BIN_DIR)/$(TARGET)

# 防止 make 将文件名误认为错误
.PHONY: all clean