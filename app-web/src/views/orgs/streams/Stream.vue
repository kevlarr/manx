<template>
  <div id="StreamView">
    <TopicList/>
    <TopicInput @submit="submitTopic"/>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import TopicInput from '@/components/streams/TopicInput.vue';
import TopicList from '@/components/streams/TopicList.vue';
import { NewTopic } from '@/types';
import Repo, { StreamRepo } from '@/lib/repo';


@Component({
  components: {
    TopicInput,
    TopicList,
  },
})
export default class Stream extends Vue {
  get stream(): StreamRepo {
    const { org, stream } = this.$route.params;
    return Repo.getOrganization(org)!.getStream(stream)!;
  }

  submitTopic(text: string) {
    const repo = this.stream;
    const topic: NewTopic = { streamId: repo.stream.id, raw: text };

    repo.createTopic(topic)
      //.then(resp => { debugger; })
      //.catch(err => { debugger; });
  }
}
</script>

<style lang='scss'>
#StreamView {
  display: flex;
  flex-direction: column;
  height: 100%;

  .TopicList {
    flex: 1;
  }

  .TopicInput {
    height: 80px;
  }
}
</style>
