import os

def print_directory_structure(path, level=0):
    indent = '    ' * level  # 根据层级设置缩进
    print(f"{indent}├── {os.path.basename(path)}")
    for root, dirs, files in os.walk(path):
        for d in dirs:
            print_directory_structure(os.path.join(root, d), level+1)
        for f in files:
            print(f"{indent}├── {f}")


# 指定要遍历的目录
project_directory = "./src"

# 打印目录结构
print_directory_structure(project_directory)