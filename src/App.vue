<template>
	<el-container>
		<el-aside width="200px">
			<MainSideBar />
		</el-aside>
		<el-main>
			<router-view v-slot="{ Component }">
				<transition name="fade" mode="out-in">
					<component :is="Component" />
				</transition>
			</router-view>
		</el-main>
	</el-container>
</template>

<script lang="ts" setup>

</script>

<style>
#app {
	font-family: Avenir, Helvetica, Arial, sans-serif;
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
}

body,
html,
#app {
	margin: 0px;
	height: 100%;
}

.el-container {
	width: 100%;
	height: 100%;
}

.el-aside,
.el-main {
	margin: 0px;
}

.el-aside {
	overflow-x: unset;
}

a {
	text-decoration: none;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
}

.fade-enter-active {
	transition: all 0.3s ease-out;
}

.fade-leave-active {
	transition: all 0.2s ease-in;
}
</style>
