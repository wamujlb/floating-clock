<script lang="ts">
	import pomodoro from '$lib/assets/pomodoro.png';
</script>

<div class="container">
	<img src={pomodoro} alt="">
</div>

<style>
	.container {
		width: 100%;
		height: 100%;
		display: flex;
    background-color: #1f1f1f;
    justify-content: center;
    align-items: center;
    border-radius: 15%;
    overflow: hidden;
	}

  img {
    width: 70%;
    height: 70%;
    display: block;
  }
</style>
