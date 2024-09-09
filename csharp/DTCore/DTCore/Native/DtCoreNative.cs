using System.Reflection;
using System.Runtime.InteropServices;

namespace DTCore.Native
{
    internal static unsafe partial class NativeMethods
    {
        static NativeMethods()
        {
            NativeLibrary.SetDllImportResolver(typeof(NativeMethods).Assembly, DllImportResolver);
        }

        static IntPtr DllImportResolver(string libraryName, Assembly assembly, DllImportSearchPath? searchPath)
        {
            if (libraryName != __DllName) return IntPtr.Zero;
            
            var path = "runtimes/";
            var prefix = "";
            var suffix = "";
            var extension = "";

            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                suffix += "-win";
                extension = ".dll";
            }
            else if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
            {
                suffix += "-macos";
                extension = ".dylib";
                prefix = "lib";
            }
            else
            {
                suffix += "-linux";
                extension = ".so";
                prefix = "lib";
            }

            if (RuntimeInformation.ProcessArchitecture == Architecture.X64)
            {
                suffix += "-amd64";
            } 
            else if (RuntimeInformation.ProcessArchitecture == Architecture.Arm64)
            {
                suffix += "-arm64";
            }
            else
            {
                throw new SystemException("Such process architecture is not support by DTCore SDK");
            }

            path += "native/" + prefix + __DllName + suffix + extension;
                
            return NativeLibrary.Load(Path.Combine(AppContext.BaseDirectory, path), assembly, searchPath);
        }
    }
}