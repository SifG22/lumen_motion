; Instalador de Lumen Motion para Windows (Inno Setup 6). Lo genera
; packaging\lumen\empaquetar.ps1 -Instalador, que pasa la version, la arquitectura y la carpeta ya
; preparada:
;   ISCC.exe /DVersion=0.0.1 /DArch=x64 /DSource=..\..\dist\LumenMotion-0.0.1-windows-x64 /DOutDir=..\..\dist lumen-motion.iss
;
; Se instala para el usuario actual (sin pedir administrador) en %LOCALAPPDATA%\Programs\Lumen Motion.
; Crea el acceso en el menu Inicio (y en el escritorio si se marca), asocia los proyectos
; .lumeneffects, las plantillas .lumeneffectstemplate y los ajustes .lumeneffectspreset (y los
; formatos anteriores .ecproj, .ecprojx, .ectemplate y .ecpreset) con su propio icono, y se
; desinstala desde Configuracion > Aplicaciones, quitando las asociaciones.

#ifndef Version
  #define Version "0.0.1"
#endif
#ifndef Arch
  #define Arch "x64"
#endif
#ifndef Source
  #define Source "..\..\dist\LumenMotion-" + Version + "-windows-" + Arch
#endif
#ifndef OutDir
  #define OutDir "..\..\dist"
#endif

[Setup]
; El AppId identifica a Lumen Motion para actualizar y desinstalar: no cambiarlo nunca.
AppId={{5E2C8A41-7B9D-4F30-9C6E-1A8D3F5B7E92}
AppName=Lumen Motion
AppVersion={#Version}
AppVerName=Lumen Motion {#Version}
AppPublisher=SifG22
AppPublisherURL=https://sifg.net
AppSupportURL=https://github.com/SifG22/lumen_motion/issues
AppUpdatesURL=https://github.com/SifG22/lumen_motion/releases
DefaultDirName={autopf}\Lumen Motion
DefaultGroupName=Lumen Motion
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
#if Arch == "arm64"
ArchitecturesAllowed=arm64
ArchitecturesInstallIn64BitMode=arm64
#else
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
#endif
MinVersion=10.0.17763
SetupIconFile=..\..\assets\app-icon\lumen-motion.ico
UninstallDisplayIcon={app}\LumenMotion.exe
UninstallDisplayName=Lumen Motion
LicenseFile={#Source}\TERMINOS.txt
OutputDir={#OutDir}
OutputBaseFilename=LumenMotion-Setup-{#Version}-{#Arch}
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
ChangesAssociations=yes
CloseApplications=yes
VersionInfoVersion={#Version}
VersionInfoCompany=SifG22
VersionInfoDescription=Instalador de Lumen Motion
VersionInfoProductName=Lumen Motion

[Languages]
Name: "es"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "en"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[Files]
Source: "{#Source}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{autoprograms}\Lumen Motion"; Filename: "{app}\LumenMotion.exe"; AppUserModelID: "SifG22.LumenMotion"
Name: "{autodesktop}\Lumen Motion"; Filename: "{app}\LumenMotion.exe"; Tasks: desktopicon

[Registry]
; Proyectos: .lumeneffects (y la copia XML .lumeneffectsx); los .ecproj/.ecprojx anteriores tambien.
; Doble clic abre el archivo en Lumen Motion; si ya esta abierta, en esa misma ventana.
Root: HKA; Subkey: "Software\Classes\.lumeneffects"; ValueType: string; ValueName: ""; ValueData: "LumenMotion.Proyecto"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.lumeneffects"; ValueType: string; ValueName: "Content Type"; ValueData: "application/x-lumeneffects"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.lumeneffectsx"; ValueType: string; ValueName: ""; ValueData: "LumenMotion.Proyecto"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.ecproj"; ValueType: string; ValueName: ""; ValueData: "LumenMotion.Proyecto"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.ecprojx"; ValueType: string; ValueName: ""; ValueData: "LumenMotion.Proyecto"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\LumenMotion.Proyecto"; ValueType: string; ValueName: ""; ValueData: "Proyecto de Lumen Motion"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\LumenMotion.Proyecto\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\LumenMotion-documento.ico"
Root: HKA; Subkey: "Software\Classes\LumenMotion.Proyecto\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\LumenMotion.exe"" ""%1"""
; Plantillas.
Root: HKA; Subkey: "Software\Classes\.lumeneffectstemplate"; ValueType: string; ValueName: ""; ValueData: "LumenMotion.Plantilla"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.ectemplate"; ValueType: string; ValueName: ""; ValueData: "LumenMotion.Plantilla"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\LumenMotion.Plantilla"; ValueType: string; ValueName: ""; ValueData: "Plantilla de Lumen Motion"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\LumenMotion.Plantilla\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\LumenMotion-documento.ico"
Root: HKA; Subkey: "Software\Classes\LumenMotion.Plantilla\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\LumenMotion.exe"" ""%1"""
; Ajustes preestablecidos (animacion, efectos, curvas de velocidad).
Root: HKA; Subkey: "Software\Classes\.lumeneffectspreset"; ValueType: string; ValueName: ""; ValueData: "LumenMotion.Ajuste"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.ecpreset"; ValueType: string; ValueName: ""; ValueData: "LumenMotion.Ajuste"; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\LumenMotion.Ajuste"; ValueType: string; ValueName: ""; ValueData: "Ajuste preestablecido de Lumen Motion"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\LumenMotion.Ajuste\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\LumenMotion-documento.ico"
Root: HKA; Subkey: "Software\Classes\LumenMotion.Ajuste\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\LumenMotion.exe"" ""%1"""
; "Abrir con" para los proyectos, videos e imagenes habituales.
Root: HKA; Subkey: "Software\Classes\Applications\LumenMotion.exe"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "Lumen Motion"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Applications\LumenMotion.exe\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\LumenMotion.exe"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\LumenMotion.exe\SupportedTypes"; ValueType: string; ValueName: ".lumeneffects"; ValueData: ""
Root: HKA; Subkey: "Software\Classes\Applications\LumenMotion.exe\SupportedTypes"; ValueType: string; ValueName: ".mp4"; ValueData: ""
Root: HKA; Subkey: "Software\Classes\Applications\LumenMotion.exe\SupportedTypes"; ValueType: string; ValueName: ".mov"; ValueData: ""
Root: HKA; Subkey: "Software\Classes\Applications\LumenMotion.exe\SupportedTypes"; ValueType: string; ValueName: ".png"; ValueData: ""
Root: HKA; Subkey: "Software\Classes\Applications\LumenMotion.exe\SupportedTypes"; ValueType: string; ValueName: ".psd"; ValueData: ""
Root: HKA; Subkey: "Software\Classes\Applications\LumenMotion.exe\SupportedTypes"; ValueType: string; ValueName: ".svg"; ValueData: ""

[Run]
Filename: "{app}\LumenMotion.exe"; Description: "{cm:LaunchProgram,Lumen Motion}"; Flags: nowait postinstall skipifsilent
