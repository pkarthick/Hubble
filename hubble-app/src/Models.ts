export class TabItem<TProps> {
  constructor(
    public id: Number,
    public icon: String,
    public path: String,
    public component: any,
    public props: TProps
  ) {}
}
