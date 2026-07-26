!macro NSIS_HOOK_PREINSTALL
  IfFileExists "$LOCALAPPDATA\Yomiage-kun\uninstall.exe" 0 legacy_install_done
  ExecWait '"$LOCALAPPDATA\Yomiage-kun\uninstall.exe" /S'
legacy_install_done:
!macroend
