<script lang="typescript">

  import MainPanel from "./MainPanel.svelte";
  import HeaderPanel from "./HeaderPanel.svelte";
  import "./app.css";

  let visible = true;

</script>

<style>
  main {
    height: 100vh;
    min-width: 95%;
  }
</style>

<main class="flex flex-col">


  <HeaderPanel on:togglesidebar={() => (visible = !visible)} />

  <MainPanel {visible} />

  <!-- <h1>Hello {name}!</h1>
  <p>May I call you {test()}?</p> -->


</main>
