import 'package:app_flowy/workspace/presentation/home/home_sizes.dart';
import 'package:app_flowy/workspace/presentation/widgets/menu/create_app_dialog.dart';
import 'package:flowy_infra/size.dart';
import 'package:flowy_infra_ui/widget/dialog/styled_dialogs.dart';
import 'package:flutter/material.dart';
import 'package:styled_widget/styled_widget.dart';

class NewAppButton extends StatelessWidget {
  final Function(String)? press;

  const NewAppButton({this.press, Key? key}) : super(key: key);
  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        border: Border(
          top: BorderSide(width: 1, color: Colors.grey.shade300),
        ),
      ),
      height: HomeSizes.menuAddButtonHeight,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.start,
        children: [
          const Icon(Icons.add_circle_rounded, size: 30),
          TextButton(
            onPressed: () async => await _showCreateAppDialog(context),
            child: const Text(
              'New App',
              style: TextStyle(
                color: Colors.black,
                fontWeight: FontWeight.bold,
                fontSize: 20,
              ),
            ),
          )
        ],
      ).padding(horizontal: Insets.l),
    );
  }

  Future<void> _showCreateAppDialog(BuildContext context) async {
    await Dialogs.showWithContext(CreateAppDialogContext(
      confirm: (appName) {
        if (appName.isNotEmpty && press != null) {
          press!(appName);
        }
      },
    ), context);
  }
}