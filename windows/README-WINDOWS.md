# QCLang for Windows

## 🚀 One-Command Installation

Open **PowerShell** or **Command Prompt** and run:

```powershell
# PowerShell (Recommended)
PowerShell -Command "iwr -useb https://qclang.dev/install.ps1 | iex"

# Or download and run
curl -o install.ps1 https://qclang.dev/install.ps1
PowerShell -ExecutionPolicy Bypass -File install.ps1

Or double-click install.bat if you downloaded it.

📦 Manual Installation
Download qclang-windows.exe from Releases

Create folder: C:\Users\YourName\AppData\Local\QCLang\bin

Place the executable there

Add that folder to your PATH:

cmd
setx PATH "%PATH%;C:\Users\YourName\AppData\Local\QCLang\bin"
🎯 Quick Start
cmd
# Verify installation
qclang --version

# Create a test quantum circuit
echo fn main() -> int { qubit q = ^|0^>; q = H(q); cbit r = measure(q); return 0; } > test.qc

# Compile it
qclang test.qc

# View the generated QASM
type test.qasm
📁 Installation Locations
The installer places files in:

Location	Purpose
%LOCALAPPDATA%\QCLang\bin\	Executables
%LOCALAPPDATA%\QCLang\examples\	Example circuits
Desktop shortcut	Quick access to examples
🔧 Troubleshooting
"qclang is not recognized"
cmd
# Refresh PATH in current session
refreshenv

# Or manually set
set PATH=%PATH%;%LOCALAPPDATA%\QCLang\bin
"Access denied" errors
Run PowerShell/CMD as Administrator.

Windows Defender warning
Click "More info" → "Run anyway" or add an exception.

Need to uninstall?
powershell
PowerShell -ExecutionPolicy Bypass -File uninstall.ps1
📝 Windows-Specific Notes
File paths with spaces
cmd
# Use quotes
qclang "C:\My Circuits\quantum.qc"
Special characters in CMD
cmd
# Use caret (^) to escape |
echo qubit q = ^|0^> > circuit.qc
PowerShell vs CMD
Use PowerShell for better experience

CMD works with the .cmd wrapper