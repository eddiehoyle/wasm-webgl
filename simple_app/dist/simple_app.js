(function() {
    var wasm;
    const __exports = {};


    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

__exports.__widl_f_set_property_CSSStyleDeclaration = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        getObject(arg0).setProperty(varg1, varg3);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_f_create_element_Document = function(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        return addHeapObject(getObject(arg0).createElement(varg1));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

function isLikeNone(x) {
    return x === undefined || x === null;
}

__exports.__widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);

    const val = getObject(arg0).getElementById(varg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_body_Document = function(arg0) {

    const val = getObject(arg0).body;
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_instanceof_Element = function(idx) {
    return getObject(idx) instanceof Element ? 1 : 0;
};

__exports.__widl_f_set_id_Element = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).id = varg1;
};

__exports.__widl_f_set_inner_html_Element = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).innerHTML = varg1;
};

__exports.__widl_f_target_Event = function(arg0) {

    const val = getObject(arg0).target;
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_instanceof_HTMLCanvasElement = function(idx) {
    return getObject(idx) instanceof HTMLCanvasElement ? 1 : 0;
};

__exports.__widl_f_get_context_HTMLCanvasElement = function(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {

        const val = getObject(arg0).getContext(varg1);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_f_set_width_HTMLCanvasElement = function(arg0, arg1) {
    getObject(arg0).width = arg1;
};

__exports.__widl_f_set_height_HTMLCanvasElement = function(arg0, arg1) {
    getObject(arg0).height = arg1;
};

__exports.__widl_instanceof_HTMLElement = function(idx) {
    return getObject(idx) instanceof HTMLElement ? 1 : 0;
};

__exports.__widl_f_style_HTMLElement = function(arg0) {
    return addHeapObject(getObject(arg0).style);
};

__exports.__widl_f_set_oninput_HTMLElement = function(arg0, arg1) {
    getObject(arg0).oninput = getObject(arg1);
};

__exports.__widl_f_set_onload_HTMLElement = function(arg0, arg1) {
    getObject(arg0).onload = getObject(arg1);
};

__exports.__widl_f_new_Image = function(exnptr) {
    try {
        return addHeapObject(new Image());
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_f_set_src_HTMLImageElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).src = varg1;
};

__exports.__widl_instanceof_HTMLInputElement = function(idx) {
    return getObject(idx) instanceof HTMLInputElement ? 1 : 0;
};

__exports.__widl_f_set_max_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).max = varg1;
};

__exports.__widl_f_set_min_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).min = varg1;
};

__exports.__widl_f_set_step_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).step = varg1;
};

__exports.__widl_f_set_type_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).type = varg1;
};

let cachedTextEncoder = new TextEncoder('utf-8');

let WASM_VECTOR_LEN = 0;

function passStringToWasm(arg) {

    const buf = cachedTextEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
}

__exports.__widl_f_value_HTMLInputElement = function(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).value);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_set_value_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).value = varg1;
};

__exports.__widl_instanceof_HTMLLabelElement = function(idx) {
    return getObject(idx) instanceof HTMLLabelElement ? 1 : 0;
};

__exports.__widl_f_append_child_Node = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).appendChild(getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_instanceof_WebGL2RenderingContext = function(idx) {
    return getObject(idx) instanceof WebGL2RenderingContext ? 1 : 0;
};

__exports.__widl_f_bind_vertex_array_WebGL2RenderingContext = function(arg0, arg1) {
    getObject(arg0).bindVertexArray(getObject(arg1));
};

__exports.__widl_f_buffer_data_with_array_buffer_view_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferData(arg1, getObject(arg2), arg3);
};

__exports.__widl_f_create_vertex_array_WebGL2RenderingContext = function(arg0) {

    const val = getObject(arg0).createVertexArray();
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_tex_image_2d_with_u32_and_u32_and_html_image_element_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, exnptr) {
    try {
        getObject(arg0).texImage2D(arg1, arg2, arg3, arg4, arg5, getObject(arg6));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

let cachegetFloat32Memory = null;
function getFloat32Memory() {
    if (cachegetFloat32Memory === null || cachegetFloat32Memory.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory;
}

function getArrayF32FromWasm(ptr, len) {
    return getFloat32Memory().subarray(ptr / 4, ptr / 4 + len);
}

__exports.__widl_f_uniform_matrix4fv_with_f32_array_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3, arg4) {
    let varg3 = getArrayF32FromWasm(arg3, arg4);
    getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, varg3);
};

__exports.__widl_f_active_texture_WebGL2RenderingContext = function(arg0, arg1) {
    getObject(arg0).activeTexture(arg1);
};

__exports.__widl_f_attach_shader_WebGL2RenderingContext = function(arg0, arg1, arg2) {
    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
};

__exports.__widl_f_bind_buffer_WebGL2RenderingContext = function(arg0, arg1, arg2) {
    getObject(arg0).bindBuffer(arg1, getObject(arg2));
};

__exports.__widl_f_bind_texture_WebGL2RenderingContext = function(arg0, arg1, arg2) {
    getObject(arg0).bindTexture(arg1, getObject(arg2));
};

__exports.__widl_f_clear_WebGL2RenderingContext = function(arg0, arg1) {
    getObject(arg0).clear(arg1);
};

__exports.__widl_f_clear_color_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
};

__exports.__widl_f_compile_shader_WebGL2RenderingContext = function(arg0, arg1) {
    getObject(arg0).compileShader(getObject(arg1));
};

__exports.__widl_f_create_buffer_WebGL2RenderingContext = function(arg0) {

    const val = getObject(arg0).createBuffer();
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_create_program_WebGL2RenderingContext = function(arg0) {

    const val = getObject(arg0).createProgram();
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_create_shader_WebGL2RenderingContext = function(arg0, arg1) {

    const val = getObject(arg0).createShader(arg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_create_texture_WebGL2RenderingContext = function(arg0) {

    const val = getObject(arg0).createTexture();
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_disable_vertex_attrib_array_WebGL2RenderingContext = function(arg0, arg1) {
    getObject(arg0).disableVertexAttribArray(arg1);
};

__exports.__widl_f_draw_elements_with_i32_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).drawElements(arg1, arg2, arg3, arg4);
};

__exports.__widl_f_enable_WebGL2RenderingContext = function(arg0, arg1) {
    getObject(arg0).enable(arg1);
};

__exports.__widl_f_enable_vertex_attrib_array_WebGL2RenderingContext = function(arg0, arg1) {
    getObject(arg0).enableVertexAttribArray(arg1);
};

__exports.__widl_f_get_attrib_location_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);
    return getObject(arg0).getAttribLocation(getObject(arg1), varg2);
};

__exports.__widl_f_get_program_info_log_WebGL2RenderingContext = function(ret, arg0, arg1) {
    const val = getObject(arg0).getProgramInfoLog(getObject(arg1));
    const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_get_program_parameter_WebGL2RenderingContext = function(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).getProgramParameter(getObject(arg1), arg2));
};

__exports.__widl_f_get_shader_info_log_WebGL2RenderingContext = function(ret, arg0, arg1) {
    const val = getObject(arg0).getShaderInfoLog(getObject(arg1));
    const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_get_shader_parameter_WebGL2RenderingContext = function(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).getShaderParameter(getObject(arg1), arg2));
};

__exports.__widl_f_get_uniform_location_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);

    const val = getObject(arg0).getUniformLocation(getObject(arg1), varg2);
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_link_program_WebGL2RenderingContext = function(arg0, arg1) {
    getObject(arg0).linkProgram(getObject(arg1));
};

__exports.__widl_f_pixel_storei_WebGL2RenderingContext = function(arg0, arg1, arg2) {
    getObject(arg0).pixelStorei(arg1, arg2);
};

__exports.__widl_f_shader_source_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);
    getObject(arg0).shaderSource(getObject(arg1), varg2);
};

__exports.__widl_f_tex_parameteri_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
    getObject(arg0).texParameteri(arg1, arg2, arg3);
};

__exports.__widl_f_use_program_WebGL2RenderingContext = function(arg0, arg1) {
    getObject(arg0).useProgram(getObject(arg1));
};

__exports.__widl_f_vertex_attrib_pointer_with_i32_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).vertexAttribPointer(arg1, arg2, arg3, arg4 !== 0, arg5, arg6);
};

__exports.__widl_f_viewport_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).viewport(arg1, arg2, arg3, arg4);
};

__exports.__widl_instanceof_Window = function(idx) {
    return getObject(idx) instanceof Window ? 1 : 0;
};

__exports.__widl_f_document_Window = function(arg0) {

    const val = getObject(arg0).document;
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_debug_4_ = function(arg0, arg1, arg2, arg3) {
    console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};

__exports.__widl_f_error_1_ = function(arg0) {
    console.error(getObject(arg0));
};

__exports.__widl_f_error_4_ = function(arg0, arg1, arg2, arg3) {
    console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};

__exports.__widl_f_info_4_ = function(arg0, arg1, arg2, arg3) {
    console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};

__exports.__widl_f_log_4_ = function(arg0, arg1, arg2, arg3) {
    console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};

__exports.__widl_f_warn_4_ = function(arg0, arg1, arg2, arg3) {
    console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};

__exports.__wbg_new_f49b071a6847bcff = function(arg0) {
    return addHeapObject(new Float32Array(getObject(arg0)));
};

__exports.__wbg_subarray_f8934b42fec7ca7c = function(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).subarray(arg1, arg2));
};

__exports.__wbg_newnoargs_43c5f57b77232284 = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
};

__exports.__wbg_call_7ac13208e630ddeb = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__wbg_new_66f4398a61abb238 = function(arg0) {
    return addHeapObject(new Uint32Array(getObject(arg0)));
};

__exports.__wbg_subarray_e15f66ea564a1778 = function(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).subarray(arg1, arg2));
};

__exports.__wbg_length_72b6e1924524141e = function(arg0) {
    return getObject(arg0).length;
};

__exports.__wbg_instanceof_Memory_ed5a1f7b9a0e05a3 = function(idx) {
    return getObject(idx) instanceof WebAssembly.Memory ? 1 : 0;
};

__exports.__wbg_buffer_efdca35786c3eb75 = function(arg0) {
    return addHeapObject(getObject(arg0).buffer);
};

__exports.__wbindgen_object_clone_ref = function(idx) {
    return addHeapObject(getObject(idx));
};

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

__exports.__wbindgen_object_drop_ref = function(i) { dropObject(i); };

__exports.__wbindgen_string_new = function(p, l) {
    return addHeapObject(getStringFromWasm(p, l));
};

__exports.__wbindgen_boolean_get = function(i) {
    let v = getObject(i);
    if (typeof(v) === 'boolean') {
        return v ? 1 : 0;
    } else {
        return 2;
    }
};

__exports.__wbindgen_debug_string = function(i, len_ptr) {
    const toString = Object.prototype.toString;
    const debug_str = val => {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debug_str(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debug_str(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
        return `${val.name}: ${val.message}
        ${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
};
const val = getObject(i);
const debug = debug_str(val);
const ptr = passStringToWasm(debug);
getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
return ptr;
};

__exports.__wbindgen_cb_drop = function(i) {
    const obj = getObject(i).original;
    dropObject(i);
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return 1;
    }
    return 0;
};

__exports.__wbindgen_cb_forget = dropObject;

__exports.__wbindgen_memory = function() { return addHeapObject(wasm.memory); };

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

__exports.__wbindgen_rethrow = function(idx) { throw takeObject(idx); };

__exports.__wbindgen_closure_wrapper783 = function(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(5);
    const d = wasm.__wbg_function_table.get(6);
    const cb = function() {
        this.cnt++;
        try {
            return f(this.a, b);

        } finally {
            if (this.cnt-- == 1) d(this.a, b);

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
};

__exports.__wbindgen_closure_wrapper785 = function(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(7);
    const d = wasm.__wbg_function_table.get(8);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            this.a = a;
            if (this.cnt-- == 1) d(this.a, b);

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
};

function freeWebClient(ptr) {

    wasm.__wbg_webclient_free(ptr);
}
/**
*/
class WebClient {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeWebClient(ptr);
    }

    /**
    * @returns {}
    */
    constructor() {
        this.ptr = wasm.webclient_new();
    }
    /**
    * @returns {void}
    */
    start() {
        return wasm.webclient_start(this.ptr);
    }
    /**
    * @param {number} arg0
    * @returns {void}
    */
    update(arg0) {
        return wasm.webclient_update(this.ptr, arg0);
    }
    /**
    * @returns {void}
    */
    render() {
        return wasm.webclient_render(this.ptr);
    }
}
__exports.WebClient = WebClient;

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

function init(path_or_module) {
    let instantiation;
    const imports = { './simple_app': __exports };
    if (path_or_module instanceof WebAssembly.Module) {
        instantiation = WebAssembly.instantiate(path_or_module, imports)
        .then(instance => {
        return { instance, module: path_or_module }
    });
} else {
    const data = fetch(path_or_module);
    if (typeof WebAssembly.instantiateStreaming === 'function') {
        instantiation = WebAssembly.instantiateStreaming(data, imports);
    } else {
        instantiation = data
        .then(response => response.arrayBuffer())
        .then(buffer => WebAssembly.instantiate(buffer, imports));
    }
}
return instantiation.then(({instance}) => {
    wasm = init.wasm = instance.exports;

});
};
self.wasm_bindgen = Object.assign(init, __exports);
})();
