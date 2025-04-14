import ctypes
import os
import sys
from pathlib import Path
from typing import Optional

class FfiError(ctypes.Structure):
    _fields_ = [
        ("message", ctypes.c_char_p),
        ("code", ctypes.c_int),
    ]

# 根据平台加载库
if sys.platform == "linux":
    lib_name = "libtex2img.so"
elif sys.platform == "darwin":
    lib_name = "libtex2img.dylib"
elif sys.platform == "win32":
    lib_name = "tex2img.dll"
else:
    raise RuntimeError("Unsupported platform")

lib_path = os.path.join(os.path.dirname(__file__), lib_name)
lib_path = os.path.join(os.path.dirname(__file__), "target", "release", lib_name) if not os.path.exists(lib_path) else lib_path
assert os.path.exists(lib_path)
lib = ctypes.CDLL(lib_path)

# 设置函数参数和返回类型
lib.pdf_to_png.argtypes = [
    ctypes.POINTER(ctypes.c_ubyte),
    ctypes.c_uint,
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.c_uint,
    ctypes.POINTER(ctypes.POINTER(FfiError)),
]
lib.pdf_to_png.restype = ctypes.c_int

lib.latex_to_png.argtypes = [
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.c_uint,
    ctypes.POINTER(ctypes.POINTER(FfiError)),
]
lib.latex_to_png.restype = ctypes.c_int

lib.free_error.argtypes = [ctypes.POINTER(FfiError)]
lib.free_error.restype = None

def pdf_to_png(pdf_data: bytes, name: str, output_dir: str, width: int = 800) -> Optional[int]:
    """
    将 PDF 数据转换为 PNG 图片
    
    :param pdf_data: PDF 文件二进制数据
    :param name: 输出文件名的前缀
    :param output_dir: 输出目录
    :param width: 输出图片宽度
    :return: 转换的页数，出错时返回 None
    """
    # 准备参数
    data_array = (ctypes.c_ubyte * len(pdf_data)).from_buffer_copy(pdf_data)
    name_bytes = name.encode('utf-8')
    path_bytes = str(Path(output_dir).absolute()).encode('utf-8')
    
    # 错误指针
    error_ptr = ctypes.POINTER(FfiError)()
    
    # 调用函数
    pages = lib.pdf_to_png(
        data_array,
        len(pdf_data),
        name_bytes,
        path_bytes,
        width,
        ctypes.byref(error_ptr),
    )
    
    # 处理错误
    if pages == -1 and error_ptr:
        error = error_ptr.contents
        error_message = error.message.decode('utf-8') if error.message else "Unknown error"
        lib.free_error(error_ptr)
        raise RuntimeError(f"PDF to PNG conversion failed: {error_message}")
    
    return pages if pages != -1 else None

def latex_to_png(latex: str, name: str, output_dir: str, width: int = 800) -> Optional[int]:
    """
    从 LaTeX 代码生成 PNG 图片
    
    :param latex: LaTeX 代码
    :param name: 输出文件名的前缀
    :param output_dir: 输出目录
    :param width: 输出图片宽度
    :return: 生成的页数，出错时返回 None
    """
    # 准备参数
    latex_bytes = latex.encode('utf-8')
    name_bytes = name.encode('utf-8')
    output_dir:Path = Path(output_dir)
    output_dir.mkdir(parents=True,exist_ok=True)
    path_bytes = str(output_dir.absolute()).encode('utf-8')
    
    # 错误指针
    error_ptr = ctypes.POINTER(FfiError)()
    
    # 调用函数
    pages = lib.latex_to_png(
        latex_bytes,
        name_bytes,
        path_bytes,
        width,
        ctypes.byref(error_ptr),
    )
    
    # 处理错误
    if pages == -1 and error_ptr:
        error = error_ptr.contents
        error_message = error.message.decode('utf-8') if error.message else "Unknown error"
        lib.free_error(error_ptr)
        raise RuntimeError(f"LaTeX to PNG conversion failed: {error_message}")
    
    return pages if pages != -1 else None

# 示例用法
if __name__ == "__main__":
    # 示例1: 从 PDF 文件转换
    # with open("example.pdf", "rb") as f:
    #     pdf_data = f.read()
    
    # pages = pdf_to_png(pdf_data, "output", "output_images")
    # print(f"Converted {pages} pages from PDF")
    
    # 示例2: 从 LaTeX 生成
    formula = r"""\[
\text{Attention}(Q, K, V) = \text{softmax}\left(\frac{QK^T}{\sqrt{d_k}}\right)V
\]"""
    latex_code = r"""
\documentclass[multi={mathpage},border=2pt, varwidth]{standalone}
\usepackage{amsmath, amssymb, amsfonts}
\usepackage{fontspec}
\usepackage{ctex}


\usepackage{xcolor} % 白底黑字增强对比度
\newenvironment{mathpage}{}{}
\begin{document}
    """ + fr"""
    
    \begin{{mathpage}}
    {formula}
    \end{{mathpage}}
    
    """ * 100 + r"\end{document}"
    
    pages = latex_to_png(latex_code, "formula", "output_images")
    print(f"Generated {pages} pages from LaTeX")