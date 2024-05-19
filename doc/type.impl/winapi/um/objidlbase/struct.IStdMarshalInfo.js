(function() {var type_impls = {
"winapi":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deref-for-IStdMarshalInfo\" class=\"impl\"><a class=\"src rightside\" href=\"src/winapi/um/objidlbase.rs.html#103-110\">source</a><a href=\"#impl-Deref-for-IStdMarshalInfo\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"winapi/um/objidlbase/struct.IStdMarshalInfo.html\" title=\"struct winapi::um::objidlbase::IStdMarshalInfo\">IStdMarshalInfo</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Target\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Target\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.78.0/core/ops/deref/trait.Deref.html#associatedtype.Target\" class=\"associatedtype\">Target</a> = <a class=\"struct\" href=\"winapi/um/unknwnbase/struct.IUnknown.html\" title=\"struct winapi::um::unknwnbase::IUnknown\">IUnknown</a></h4></section></summary><div class='docblock'>The resulting type after dereferencing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winapi/um/objidlbase.rs.html#103-110\">source</a><a href=\"#method.deref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.78.0/core/ops/deref/trait.Deref.html#tymethod.deref\" class=\"fn\">deref</a>(&amp;self) -&gt; &amp;<a class=\"struct\" href=\"winapi/um/unknwnbase/struct.IUnknown.html\" title=\"struct winapi::um::unknwnbase::IUnknown\">IUnknown</a></h4></section></summary><div class='docblock'>Dereferences the value.</div></details></div></details>","Deref","winapi::um::objidlbase::LPSTDMARSHALINFO"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IStdMarshalInfo\" class=\"impl\"><a class=\"src rightside\" href=\"src/winapi/um/objidlbase.rs.html#103-110\">source</a><a href=\"#impl-IStdMarshalInfo\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"winapi/um/objidlbase/struct.IStdMarshalInfo.html\" title=\"struct winapi::um::objidlbase::IStdMarshalInfo\">IStdMarshalInfo</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.GetClassForHandler\" class=\"method\"><a class=\"src rightside\" href=\"src/winapi/um/objidlbase.rs.html#103-110\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"winapi/um/objidlbase/struct.IStdMarshalInfo.html#tymethod.GetClassForHandler\" class=\"fn\">GetClassForHandler</a>(\n    &amp;self,\n    dwDestContext: <a class=\"type\" href=\"winapi/shared/minwindef/type.DWORD.html\" title=\"type winapi::shared::minwindef::DWORD\">DWORD</a>,\n    pvDestContext: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/core/primitive.pointer.html\">*mut </a><a class=\"enum\" href=\"winapi/ctypes/enum.c_void.html\" title=\"enum winapi::ctypes::c_void\">c_void</a>,\n    pClsid: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/core/primitive.pointer.html\">*mut </a><a class=\"type\" href=\"winapi/shared/guiddef/type.CLSID.html\" title=\"type winapi::shared::guiddef::CLSID\">CLSID</a>\n) -&gt; <a class=\"type\" href=\"winapi/um/winnt/type.HRESULT.html\" title=\"type winapi::um::winnt::HRESULT\">HRESULT</a></h4></section></div></details>",0,"winapi::um::objidlbase::LPSTDMARSHALINFO"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Interface-for-IStdMarshalInfo\" class=\"impl\"><a class=\"src rightside\" href=\"src/winapi/um/objidlbase.rs.html#103-110\">source</a><a href=\"#impl-Interface-for-IStdMarshalInfo\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"winapi/trait.Interface.html\" title=\"trait winapi::Interface\">Interface</a> for <a class=\"struct\" href=\"winapi/um/objidlbase/struct.IStdMarshalInfo.html\" title=\"struct winapi::um::objidlbase::IStdMarshalInfo\">IStdMarshalInfo</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.uuidof\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winapi/um/objidlbase.rs.html#103-110\">source</a><a href=\"#method.uuidof\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winapi/trait.Interface.html#tymethod.uuidof\" class=\"fn\">uuidof</a>() -&gt; <a class=\"struct\" href=\"winapi/shared/guiddef/struct.GUID.html\" title=\"struct winapi::shared::guiddef::GUID\">GUID</a></h4></section></div></details>","Interface","winapi::um::objidlbase::LPSTDMARSHALINFO"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()