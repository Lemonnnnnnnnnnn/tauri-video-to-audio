<script lang="ts">
  import clsx from "clsx";
  export let show : boolean;
</script>

<main>
  <div class={clsx("mask", show && "active")} />
  <div class={clsx("icon-wrapper", show && "active")}>
    <svg
      class={clsx("icon", show && "active")}
      viewBox="0 0 1024 1024"
      version="1.1"
      xmlns="http://www.w3.org/2000/svg"
      data-darkreader-inline-fill=""
      width="200"
      fill="white"
      height="200"
      ><path
        d="M988 548c-19.9 0-36-16.1-36-36 0-59.4-11.6-117-34.6-171.3a440.45 440.45 0 0 0-94.3-139.9 437.71 437.71 0 0 0-139.9-94.3C629 83.6 571.4 72 512 72c-19.9 0-36-16.1-36-36s16.1-36 36-36c69.1 0 136.2 13.5 199.3 40.3C772.3 66 827 103 874 150c47 47 83.9 101.8 109.7 162.7 26.7 63.1 40.2 130.2 40.2 199.3 0.1 19.9-16 36-35.9 36z"
      /></svg
    >
  </div>
</main>

<style>
  .mask {
    height: 100vh;
    background-color: rgb(0, 0, 0);
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    z-index: -1;
    opacity: 0;
    transition: all .4s ease-in;
  }

  .mask.active {
    opacity: 0.7;
    z-index: 10;
  }

  .icon-wrapper {
    height: 100vh;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    z-index: -1;
    display: grid;
    place-items: center;
    background-color: transparent;
  }

  .icon {
    height: 30px;
    width: 30px;
    width: 100%;
    z-index: -1;
    transition: all .4s ease-in;
    transform: scale(0);
    animation: .7s linear 0s infinite spin;
  }

  .icon.active {
    z-index: 20;
    transform: scale(1);
  }

  .icon-wrapper.active {
    z-index: 20;
  }

  @keyframes spin{
    0%{
        transform: rotate(0deg);
    }
    100%{
        transform: rotate(360deg);
    }
  }

</style>
