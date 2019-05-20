<template>
  <div class="TopicInput">
    <TextArea
      placeholder="Say something..."
      v-model="value"
      @shift-enter="submit"
      @escape="clear"
    />
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';

@Component
export default class TopicInput extends Vue {
  value: string = '';

  submit() {
    this.$emit('submit', this.value);
    this.clear();
  }

  clear() {
    this.value = '';
  }
}
</script>

<style lang='scss'>
.TopicInput {
  & > .TextArea {
    height: 100%;

    & > textarea {
      height: 100%;
    }
  }
}
</style>
