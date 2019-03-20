<template>
  <div id="OrganizationsView">
    <Nav/>
    <div class="organizations-content">
      <!-- Assign key so orgId change can re-create component -->
      <router-view :key="$route.params.orgId"/>
    </div>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import Nav from '@/components/organizations/OrganizationsNav.vue';

@Component({
  components: {
    Nav,
  },
})
export default class extends Vue {
}
</script>

<style lang='scss'>
#OrganizationsView {
  display: flex;
  height: 100%;
  flex-direction: column;

  .organizations-content {
    height: 100%;
  }
}
</style>
