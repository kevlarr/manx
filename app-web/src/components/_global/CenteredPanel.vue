<template>
  <div class="CenteredPanel">
    <slot></slot>
  </div>
</template>

<style lang='scss'>
.CenteredPanel {
  margin: 0 auto;
  max-width: 400px;
  padding-top: 100px;
  text-align: center;
}
</style>
