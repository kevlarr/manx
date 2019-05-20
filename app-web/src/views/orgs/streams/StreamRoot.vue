 <template>
   <router-view :key="$route.params.stream"/>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Route } from 'vue-router';
import { Next } from '@/types';
import { slugerize } from '@/lib/helpers';
import Repo from '@/lib/repo';


const findStream = (org: string, stream: string) =>
  Repo.getOrganization(org)!.getStream(stream)!.stream;

const verifyStream = (to: Route, next: Next) =>
  findStream(to.params.org, to.params.stream)
    ? next()
    : redirectToGlobal(to, next);

const redirectToGlobal = (to: Route, next: Next) => {
  const global = Repo
    .getOrganization(to.params.org)!
    .getGlobalStream()!
    .stream;

  const params = { ...to.params, stream: global.shortId };

  next({ name: 'stream', params });
}


@Component
export default class Stream extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    console.info('StreamRoot:beforeEnter');
    verifyStream(to, next);
  }

  beforeRouteUpdate(to: Route, from: Route, next: Next) {
    console.info('StreamRoot:beforeUpdate');

    if (to.params.stream === from.params.stream) {
      to.name === 'streamRoot' ? next(false) : next();
      return;
    }

    verifyStream(to, next);
  }

  /**
   * Re-slugerizes the stream name and updates route to child path
   * if it's the stream root
   */

  created() {
    console.info('StreamRoot:created');

    const { org, stream: shortId } = this.$route.params;
    const stream = findStream(org, shortId);
    const newName = stream.global ? 'Global' : slugerize(stream.name);

    if (newName === this.$route.params.streamName) { return; }

    const params = { ...this.$route.params, streamName: newName };
    const toName = this.$route.name === 'streamRoot'
      ? 'stream'
      : this.$route.name;

    console.info('\tUpdating Stream name');
    this.$router.push({ name: toName, params });
  }
}
</script>
