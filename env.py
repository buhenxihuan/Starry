import os

file_path = os.environ.get('GITHUB_ENV', None)
if file_path is None:
    raise OSError('Environment file not found')

# 读取文件内容并打印
with open(file_path, 'r') as gh_envs:
    for line in gh_envs:
        print(line.strip())  # 打印每一行内容（移除开头和结尾的空白符）