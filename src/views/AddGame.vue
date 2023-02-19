<script lang="ts" setup>
// !为了不让 submit_handler 报错，这里不使用 lang="ts"
import { DocumentAdd } from "@element-plus/icons-vue";
import {
    Check,
    RefreshRight,
    Download,
    MagicStick,
} from "@element-plus/icons-vue";
import { ElNotification } from "element-plus";
import { reactive, ref } from "vue";
import { useRouter } from "vue-router";
const router = useRouter();


const game_name = ref("") // 写入游戏名
const save_path = ref("") // 选择游戏存档目录
const game_path = ref("") // 选择游戏启动程序
const game_icon_src = ref("https://shadow.elemecdn.com/app/element/hamburger.9cf7b091-55e9-11e9-a976-7f4d0b07eef6.png")
const buttons = [
    {
        text: "修改已保存的配置",
        type: "",
        icon: MagicStick,
        method: change,
    },
    {
        text: "自动识别本地游戏",
        type: "primary",
        icon: Download,
        method: search_local,
    },
    {
        text: "保存当前编辑的配置",
        type: "success",
        icon: Check,
        method: save,
    },
    {
        text: "重置当前配置",
        type: "danger",
        icon: RefreshRight,
        method: reset,
    },
] as const;


function choose_save_directory() {
    // TODO:选择游戏存档目录
}
function choose_executable_file() {
    // TODO:选择游戏启动程序
}
function choose_game_icon() {
    // TODO:选择游戏图标地址
    // TODO:需要优化图片选择，自由切割和压缩到指定大小
}
function submit_handler(button_method: Function) {
    // TODO:映射按钮的ID和他们要触发的方法
    button_method();
}
function search_local() {
    // TODO:导入已有配置
    ElNotification({
        type: "warning",
        message: "--WIP-- 这个功能尚未完成",
    });
}
function change() {
    router.push("/change-game-info");
}
function save() {
    if (game_name.value == "" || save_path.value == "") {
        ElNotification({
            type: "error",
            message: "请至少输入游戏名和存档路径"
        })
        return
    }
    // TODO保存当前配置
    console.log("保存当前编辑的配置");
    // ipcRenderer.send("add_game", {
    // 	game_name: game_name.value,
    // 	save_path: save_path.value,
    // 	icon: game_icon_src.value,
    // 	game_path: game_path.value,
    // });
}
function reset() {
    // 重置当前配置
    game_name.value = "";
    save_path.value = "";
    game_path.value = "";
    game_icon_src.value =
        "https://shadow.elemecdn.com/app/element/hamburger.9cf7b091-55e9-11e9-a976-7f4d0b07eef6.png";
    ElNotification({
        title: "提示",
        message: "重置成功",
        type: "success",
        duration: 1000,
    });
}
</script>

<template>
    <div class="select-container">
        <el-main>
            <el-card class="game-info">
                <div class="top-part">
                    <img class="game-icon" :src="game_icon_src" />
                </div>
                <div style="padding: 14px" class="input-container">
                    <div class="bottom">
                        <el-button @click="choose_game_icon()" type="default" class="button" disabled>
                            选择游戏图标(请使用方形图片)
                        </el-button>
                        <el-input v-model="game_name" placeholder="请输入游戏名（必须）">
                            <template #prepend>
                                游戏名称
                            </template>
                        </el-input>
                        <el-input v-model="save_path" placeholder="请选择存档路径（必须）">
                            <template #prepend>
                                存档目录
                            </template>
                            <template #append>
                                <el-button @click="choose_save_directory()">
                                    <el-icon>
                                        <document-add />
                                    </el-icon>
                                </el-button>
                            </template>
                        </el-input>
                        <el-input v-model="game_path" placeholder="请选择游戏启动文件（非必须）">
                            <template #prepend>
                                启动文件
                            </template>
                            <template #append>
                                <el-button @click="choose_executable_file()">
                                    <el-icon>
                                        <document-add />
                                    </el-icon>
                                </el-button>
                            </template>
                        </el-input>
                    </div>
                </div>
            </el-card>
        </el-main>
        <el-footer>
            <el-container class="submit-bar">
                <el-tooltip v-for="button in buttons" :key="button.text" :content="button.text" placement="top">
                    <el-button @click="submit_handler(button.method)" :type="button.type" circle>
                        <el-icon>
                            <component :is="button.icon" />
                        </el-icon>
                    </el-button>
                </el-tooltip>
            </el-container>
        </el-footer>
    </div>
</template>

<style scoped>
.select-container {
    height: 100%;
    width: 100%;
}

.el-main {
    height: 90%;
    overflow: hidden;
}

.game-info {
    height: 99%;
}

.top-part {
    height: 200px;
}

.game-icon {
    float: left;
    height: 200px;
    width: 200px;
}

.el-input {
    margin-top: 10px;
}

.submit-bar {
    justify-content: flex-end;
}
</style>