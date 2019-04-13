<template>
  <div>
    <section>
      <TextInput label="my input" v-model="textModel" autofocus/>
      <TextInput
        label="asdf"
        v-model="textModel"
        classNames="some classes"
        placeholder="some text"
        err-msg="some error"
        disabled
      />
    </section>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import Component from 'vue-class-component';

@Component
export default class extends Vue {
  data() {
    return {
      textModel: '',
    };
  }
}
</script>
