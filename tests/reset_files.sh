DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
DIR=$(pwd)

cd "$DIR/files" || return
rm ./* ; touch test{1..5}.txt
