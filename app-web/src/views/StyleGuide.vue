<template>
  <div id="StyleGuide">
    <nav>
      <router-link :to="{ name: 'styleGuideText' }">Text</router-link> |
      <router-link :to="{ name: 'styleGuideComponents' }">Components</router-link>
    </nav>
    <hr/>
    <main>
      <router-view/>
    </main>
  </div>
</template>

<style scoped lang='scss'>
nav, main {
  padding: 10px;
}
</style>
