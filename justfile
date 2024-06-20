
_default: && system-info
  @just --list --unsorted

test:
  cd server && cargo test

run_server:
  cd server && cargo r

system-info:
  @echo ""
  @echo "------------------------------------------------------------------------------------------------"
  @echo ""
  @echo "==> System infos :" 
  @echo "  -  os/arch: {{os()}}/{{arch()}}".
  @echo ""
