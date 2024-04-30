# 介绍
> 这是一个rust wasm在web worker中的demo
# 使用

打包
```shell
wasm-pack build --target no-modules
```
复制pkg目录到程序静态目录中，比如nuxt项目的public目录
```shell
cp -rf pkg /path/to/nuxt/public/js
```

worker文件，位置放pkg上级 `public/js/worker.js`
```js
self.importScripts("./pkg/rust_wasm_md5.js")
const {md5} = wasm_bindgen;
self.onmessage = async ({data}) => {
    await wasm_bindgen(
        "./pkg/rust_wasm_md5_bg.wasm"
    )
    let value = md5(data, 1024 * 1024 * 8, (process) => {
        self.postMessage({process})
    })
    self.postMessage({value})
}
```
主线程调用
```js
//定义方法
const wasm_md5 = ({file, on_process}) => {
    return new Promise(async (resolve, reject) => {
        //判断是否支持wasm
        try {
            if (!WebAssembly) {
                reject(new Error('don\'t support WebAssembly'))
                return
            }
        } catch (e) {
            reject(new Error('don\'t support WebAssembly'))
            return
        }
        let worker = new Worker('/js/worker.js')
        let bytes = await new Promise((r) => {
            let reader = new FileReader()
            reader.onload = function (e) {
                r(new Uint8Array(e.target.result))
            }
            reader.readAsArrayBuffer(file)
        })
        worker.onerror = (e) => {
            worker.terminate()
        }
        worker.onmessage = ({data}) => {
            if (data.value) {
                worker.terminate()
                resolve({ok: true, value: data.value})
            } else {
                let process = data.process.toFixed(2)
                if (on_process) {
                    on_process({process})
                }
            }
        }
        worker.postMessage(bytes, [bytes.buffer])
    })
}

const upload = async () => {
      let file = document.getElementById('file').files[0]
      let data = await wasm_md5({
        file,
        on_process: ({process})=>{
          console.log(process)
        },
      })
      console.log('md5:',data.value)
}
```