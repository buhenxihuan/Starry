import os
print(os.environ)
print(os.environ.get("ARCH"))
env=os.getenv('GITHUB_ENV')
print(env)
arch=os.getenv('ARCH')
print(arch)