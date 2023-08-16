<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow } from "@tauri-apps/api/window";
  import { convertFileSrc } from '@tauri-apps/api/tauri';
  import Loading from "./lib/Loading.svelte";

  appWindow.listen("tauri://file-drop" ,  (event) => {
    const path = event['payload'][0]
    if(!checkFileType(path)){
      return alert("请传入mp4文件")
    }
    videoUrl = path
  })

  const checkFileType = (path : string) => {
    const reg = /^.*\.mp4$/
    return reg.test(path)
  }

  let videoUrl = "";
  let customSec = 0;
  let show = false

  const handleClick = async (num: number) => {
    if(!checkForm(num)) return 

    show = true   
    await invoke("transform", { num: String(num) , videoUrl });
    show = false

    alert("成功！！！")
  };

  const checkForm = (num : number) => {
    if(!videoUrl){
      alert("请先传入mp4文件")
      return false
    }

    if(num < 0){
      alert("空白时间不能为负数")
      return false
    }

    return true
  }
</script>

<main>
  <div class="container" aria-hidden="true">
    <video width="320" height="240" controls src={convertFileSrc(videoUrl)}
      ><track kind="captions" />
    </video>

    <div class="btns">
      <button on:click={() => handleClick(0)}>转换为音频</button>
      <button on:click={() => handleClick(5)}>转换为音频，前面加五秒空白</button
      >
      <button on:click={() => handleClick(10)}
        >转换为音频，前面加十秒空白</button
      >
      <button on:click={() => handleClick(customSec)} class="custom-button"
        >转换为音频，前面加
        <svg
          on:click|stopPropagation={() => (customSec -= 1)}
          aria-hidden
          class="icon"
          viewBox="0 0 1024 1024"
          version="1.1"
          fill="currentColor"
          xmlns="http://www.w3.org/2000/svg"
          data-darkreader-inline-fill=""
          width="200"
          height="200"
          ><path
            d="M696 480H328c-4.4 0-8 3.6-8 8v48c0 4.4 3.6 8 8 8h368c4.4 0 8-3.6 8-8v-48c0-4.4-3.6-8-8-8z"
          /><path
            d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64z m0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z"
          /></svg
        >
        <input
          bind:value={customSec}
          min={0}
          type="number"
          on:click|stopPropagation
        />
        <svg
          on:click|stopPropagation={() => (customSec += 1)}
          aria-hidden
          class="icon"
          viewBox="0 0 1024 1024"
          version="1.1"
          xmlns="http://www.w3.org/2000/svg"
          data-darkreader-inline-fill=""
          width="200"
          fill="currentColor"
          height="200"
          ><path
            d="M512 56.888889C260.636444 56.888889 56.888889 260.636444 56.888889 512s203.747556 455.111111 455.111111 455.111111 455.111111-203.747556 455.111111-455.111111S763.363556 56.888889 512 56.888889z m0 796.444444c-188.216889 0-341.333333-153.116444-341.333333-341.333333S323.783111 170.666667 512 170.666667s341.333333 153.116444 341.333333 341.333333-153.116444 341.333333-341.333333 341.333333z"
          /><path
            d="M683.207111 455.111111H568.888889v-114.318222a56.32 56.32 0 0 0-56.32-56.32h-1.109333a56.32 56.32 0 0 0-56.32 56.32V455.111111h-114.318223a56.32 56.32 0 0 0-56.32 56.32v1.109333a56.32 56.32 0 0 0 56.32 56.32h114.346667v114.318223a56.32 56.32 0 0 0 56.32 56.32h1.109333a56.32 56.32 0 0 0 56.32-56.32v-114.346667h114.318223a56.32 56.32 0 0 0 56.32-56.32v-1.109333a56.32 56.32 0 0 0-56.348445-56.32z"
          /></svg
        >
        秒空白
      </button>
    </div>
  </div>
  <Loading show={show} />
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    font-family: Roboto, -apple-system, BlinkMacSystemFont, "Helvetica Neue",
      "Segoe UI", "Oxygen", "Ubuntu", "Cantarell", "Open Sans", sans-serif;
    color: #86a5b1;
    background-color: #2f3241;
    height: 100vh;
  }

  .custom-button {
    display: flex;
    align-items: center;
    column-gap: 10px;
  }

  .icon {
    display: inline-block;
    width: 30px;
    height: 30px;
    line-height: 30px;
    box-sizing: border-box;
    border-radius: 50%;
    transition: all 0.2s ease-in;
  }

  .icon:hover {
    color: rgb(110, 171, 108);
  }

  .icon:active {
    color: #646cff;
  }

  .container {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .btns {
    margin-left: 40px;
    display: flex;
    flex-direction: column;
    row-gap: 20px;
  }

  input {
    width: 3ch;
    background-color: transparent;
    color: white;
  }
</style>
