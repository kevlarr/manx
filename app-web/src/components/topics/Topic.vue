<template>
  <div class="Topic">
    <div class="content" v-html="topic.rendered"></div>
    <p class="meta">{{ repo.comments.length }} comment(s)</p>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Observer } from 'mobx-vue';
import { TopicRepo } from '@/lib/repo';

@Observer
@Component({
  props: {
    repo: TopicRepo,
  },
})
export default class Topic extends Vue {
  get topic() {
    return this.$props.repo.topic;
  }
}
</script>

<style lang='scss'>
.Topic {
  padding: 10px;
}

.meta {
  color: #999;
  font-size: 14px;
}

.content {
  margin-bottom: 10px;
  * {
    box-sizing: border-box;
  }

  p {
    margin: 0;
  }

  code {
    background: #333;
    color: #fff;
    display: inline-block;
    font-size: 16px;
    padding: 1px 4px;
    transform: translatey(-1px);
  }

  pre {
    background: #333;
    color: #fff;
    padding: 4px 8px;
  }

  pre code {
    padding: 0;
  }
}
</style>
