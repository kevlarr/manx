<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png">
    <h1>welcome to manx</h1>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import Component from 'vue-class-component';
import HelloWorld from '@/components/HelloWorld.vue'; // @ is an alias to /src

@Component
export default class Home extends Vue {
};
</script>
